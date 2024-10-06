//! Control the surface failure determination strategy.
//! 
//! Surface failure can be obtained by :
//! 
//! * Loading custom surface
//! * SLBL (fast matrix solving)
//! * SLBL (customizable iterative solving)

use crate::prelude::{self, PropOnSection, Dem1D};
use std::f64::consts::PI;
use std::sync::Arc;
mod disp;

/// Global struct for handling slides.
/// 
/// Given a reference to a DEM, it can reconstruct a failure surface with the SLBL method.
#[derive(Debug)]
pub struct Slide {
    dem: Arc<Dem1D>,
    config: SlideConfig,
    pub surface: Option<PropOnSection>,
    pub slope: Option<PropOnSection>,
    pub pilar_slope: Option<(PropOnSection, PropOnSection)>,
    pub vector_slope: Option<(PropOnSection, PropOnSection)>,
    pub main_dir: i8,
    pub ratio: f64,
} 
// TODO impl update signal
// TODO suppr
impl Slide {
    /// Initialize the slide
    pub fn new(dem: Arc<Dem1D>, config: SlideConfig) -> Slide {
        let local_dem = dem.clone();
        Slide {
            dem: dem,
            config,
            surface: None,
            slope: None,
            pilar_slope: None,
            vector_slope: None,
            main_dir: -sign_to_coeff(local_dem.z.last().unwrap() - local_dem.z.first().unwrap()),
            ratio: 1.,
        }
    }

    /// Compute the failure surface
    pub(crate) fn update_slide(&mut self) {
        self.surface = Some(compute_slide(&self.config, &self.dem));
    }

    /// Compute the slope associated with the slide failure surface 
    pub(crate) fn update_slope_and_vec(&mut self) {
        self.slope = Some(disp::slope(&self.dem, self.surface.as_ref().unwrap()));
        self.vector_slope = Some(disp::slope2vec(self.slope.as_ref().unwrap(), self.main_dir, self.config.first_pnt, self.config.last_pnt));
    }

    /// Compute the projection of the position of the points of the failure surface on the DEM surface
    /// 
    /// The displacement direction is considered to be along the failure surface, but the displacement in 
    /// monitoring is seen on surface. The displacement is put on the surface considering equel displacement
    /// value on a perpendicular slide of the landslide.
    pub(crate) fn update_pilar(&mut self) {
        self.pilar_slope = Some(disp::pilar_slope(&self.config, self.surface.as_ref().unwrap(), self.slope.as_ref().unwrap(), &self.dem));
    }

    
}

/// Method selected to compute the SLBL failure surface
/// 
/// * RoutineSimple: Iterative method with basic ending conditions
/// * RoutineThreshold: Iterative method with more ending conditions (slope limit, elevation limit)
/// * ExactMatrix: Resolution by matrix inversion, gives the exact solution but don't implement additionnal limits
#[derive(Debug)]
pub enum SlideMethod {
    RoutineSimple,
    RoutineThreshold,
    ExactMatrix,
}

/// Parameters needed for SLBL computation, depending on the chosen method
#[derive(Debug)]
pub struct SlideConfig {
    method: SlideMethod,
    first_pnt: usize,
    last_pnt: usize,
    tol: f64,
    n_it: Option<usize>,
    elevation_min: Option<f64>,
    slope_max: Option<f64>,
}

impl SlideConfig {
    /// Initializing the slide configuration parameters
    pub fn new(method: SlideMethod, first_pnt: usize, last_pnt: usize, tolerance: f64) -> SlideConfig{
        SlideConfig{
            method,
            first_pnt, 
            last_pnt, 
            tol: tolerance, 
            n_it: None, 
            elevation_min: None, 
            slope_max: None,
        }
    }
}

/// Select the function to compute SLBL given a configuration
fn compute_slide(config: &SlideConfig, dem: &prelude::Dem1D) -> prelude::PropOnSection {
    let z_slide: Vec<f64> = match config.method {
        SlideMethod::RoutineSimple => slbl_routine_simple(dem, config),
        //SlideMethod::RoutineThreshold => (),
        SlideMethod::ExactMatrix => slbl_matrix(dem, config),
        _ => panic!("SlideMethod is invalid in compute_slide."),
    };
    let name: String = "slide".to_string();
    PropOnSection::new(name, z_slide)
}
/// Compute the SLBL surface with matrix inversion
/// 
/// Implement with fast resolution of tridiagonal matrix
fn slbl_matrix(dem: &prelude::Dem1D, config: &SlideConfig) -> Vec<f64> {
    let dim: usize = config.last_pnt - config.first_pnt - 1;
    let sub_diag: Vec<f64> = vec![-0.5 ; dim-1];
    let mut main_diag: Vec<f64> = vec![1. ; dim];
    let mut rhs: Vec<f64> = vec![-config.tol ; dim];
    rhs[0] += dem.z[config.first_pnt] / 2.;
    rhs[dim-1] += dem.z[config.last_pnt] / 2.;
    let m_result = tridiag_matrix_non_conservative(dim, &sub_diag, &mut main_diag, &sub_diag, &mut rhs);
    let mut result = dem.z.to_owned();
    for i in (config.first_pnt + 1)..config.last_pnt {
        result[i] = m_result[i - config.first_pnt - 1]
    }
    result
}

/// Compute the SLBL surface with the simple iterative method
fn slbl_routine_simple(dem: &prelude::Dem1D, config: &SlideConfig) -> Vec<f64> {
    //z_topo: &Vec<f64>, n_it: usize, tol: f64
    let mut m_result = dem.z[config.first_pnt..=config.last_pnt].to_owned();
    let mut result = dem.z.to_owned();
    let Some(it) = config.n_it else {
        panic!("Missing parameter (n_it) for slbl_routine_simple");
    };
    for _ in 0..it {
        let z_temp = m_result.clone();
        for i in 1..m_result.len()-1 {
            let local_mean = (z_temp[i-1] + z_temp[i+1]) / 2. - config.tol;
            if local_mean < m_result[i] {
                m_result[i] = local_mean;
            }
        }
    }
    for i in (config.first_pnt + 1)..config.last_pnt {
        result[i] = m_result[i - config.first_pnt + 1]
    }
    result
}

/// Compute the SLBL surface with the iterative method and additionnal thresholds
fn slbl_routine_thresholds(z_topo: &Vec<f64>, n_it: usize, tol: f64, elevation_min: Option<f64>, slope_max: Option<(f64, f64)>) -> (Vec<f64>, usize) {
    let mut z_slbl = z_topo.clone();
    let mut current_it: usize = 0;
    'routine : while current_it < n_it {
        let z_temp = z_slbl.clone();
        '_section : for i in 1..z_slbl.len()-1 {
            let local_mean = (z_temp[i-1] + z_temp[i+1]) / 2. - tol;
            if let Some(elevation_min) = elevation_min {
                if local_mean < elevation_min {
                    break 'routine; // vs continue 'section
                }
            }
            if let Some((slope_max, x_spacing)) = slope_max { // not only left slope
                if (local_slope(z_slbl[i-1], local_mean, x_spacing),
                local_slope(local_mean, z_slbl[i+1], x_spacing)) > (slope_max, slope_max) {
                    break 'routine;
                 }
            }
            if local_mean < z_slbl[i] {
                z_slbl[i] = local_mean;
            }
        }
        current_it += 1;
    }
    (z_slbl, current_it) // return current_it as it can be less than n_it
}

/// Compute the slope between two succesive points, given their respective values and the spacing between them
fn local_slope(first_point: f64, second_point: f64, spacing: f64) -> f64 {
    let result: f64 = ((first_point - second_point).abs() / spacing).atan() * 180. / PI; // degrees
    result
}

/// Sign function which gives
/// 
/// * `1` if the number is positive or null
/// * `-1` if the number is negative
fn sign_to_coeff(number: f64) -> i8 {
    match number {
        i if i >= 0. => 1 as i8,
        _ => -1 as i8,
    }
}

/// Solver of linear system A.X=B where A is tridiagonal
fn tridiag_matrix_non_conservative(dim: usize, low_d: &Vec<f64>, main_d: &mut Vec<f64>, upp_d: &Vec<f64>, rhs: &mut Vec<f64>) -> Vec<f64>  {
    // Check that the sizes are matching
    assert_eq!(low_d.len(), dim - 1, "Low_d doesn't match the problem's dimension.");
    assert_eq!(upp_d.len(), dim - 1, "Upp_d doesn't match the problem's dimension.");
    assert_eq!(main_d.len(), dim, "Main_d doesn't match the problem's dimension.");
    assert_eq!(rhs.len(), dim, "Rhs doesn't match the problem's dimension.");
    // Update coefficient for computation
    // Initial values for main_d and rhs are not conserved
    for i in 1..dim {
        let temp_value = low_d[i-1] / main_d[i-1];
        main_d[i] -= temp_value * upp_d[i-1];
        rhs[i] -= temp_value * rhs[i-1];
    }
    // Back substitution to obtain the result
    let mut result: Vec<f64> = vec![];
    result.push(rhs[dim-1] / main_d[dim-1]);
    for i in 1..dim{
        result.push((rhs[dim-1-i] - upp_d[dim-1-i] * result[i-1]) / main_d[dim-1-i]);
    }
    result.reverse();
    result
}

#[cfg(test)]
mod tests {

}