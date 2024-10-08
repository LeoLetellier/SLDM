use disp::pilar_slope;

use crate::types::*;
use std::{f32::consts::PI, ops::Deref};
mod disp;


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
    tol: f32,
    n_it: Option<usize>,
    elevation_min: Option<f32>,
    slope_max: Option<f32>,
}

impl SlideConfig {
    /// Initializing the slide configuration parameters
    pub fn new(method: SlideMethod, first_pnt: usize, last_pnt: usize, tolerance: f32) -> SlideConfig{
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
fn compute_slide(config: &SlideConfig, dem: &Dem1D) -> Vec<f32> {
    let z_slide: Vec<f32> = match config.method {
        SlideMethod::RoutineSimple => slbl_routine_simple(dem, config),
        //SlideMethod::RoutineThreshold => (),
        SlideMethod::ExactMatrix => slbl_matrix(dem, config),
        _ => panic!("SlideMethod is invalid in compute_slide."),
    };
    z_slide
}

/// Compute the SLBL surface with matrix inversion
/// 
/// Implement with fast resolution of tridiagonal matrix
fn slbl_matrix(dem: &Dem1D, config: &SlideConfig) -> Vec<f32> {
    let dim: usize = config.last_pnt - config.first_pnt - 1;
    let sub_diag: Vec<f32> = vec![-0.5 ; dim-1];
    let mut main_diag: Vec<f32> = vec![1. ; dim];
    let mut rhs: Vec<f32> = vec![-config.tol ; dim];
    rhs[0] += dem.surface.z[config.first_pnt] / 2.;
    rhs[dim-1] += dem.surface.z[config.last_pnt] / 2.;
    let m_result = tridiag_matrix_non_conservative(dim, &sub_diag, &mut main_diag, &sub_diag, &mut rhs);
    let mut result = dem.surface.z.to_owned();
    for i in (config.first_pnt + 1)..config.last_pnt {
        result[i] = m_result[i - config.first_pnt - 1]
    }
    result
}

/// Compute the SLBL surface with the simple iterative method
fn slbl_routine_simple(dem: &Dem1D, config: &SlideConfig) -> Vec<f32> {
    //z_topo: &Vec<f32>, n_it: usize, tol: f32
    let mut m_result = dem.surface.z[config.first_pnt..=config.last_pnt].to_owned();
    let mut result = dem.surface.z.to_owned();
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
fn slbl_routine_thresholds(z_topo: &Vec<f32>, n_it: usize, tol: f32, elevation_min: Option<f32>, slope_max: Option<(f32, f32)>) -> (Vec<f32>, usize) {
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
fn local_slope(first_point: f32, second_point: f32, spacing: f32) -> f32 {
    let result: f32 = ((first_point - second_point).abs() / spacing).atan() * 180. / PI; // degrees
    result
}

/// Sign function which gives
/// 
/// * `1` if the number is positive or null
/// * `-1` if the number is negative
fn sign_to_coeff(number: f32) -> i8 {
    match number {
        i if i >= 0. => 1 as i8,
        _ => -1 as i8,
    }
}

/// Solver of linear system A.X=B where A is tridiagonal
fn tridiag_matrix_non_conservative(dim: usize, low_d: &Vec<f32>, main_d: &mut Vec<f32>, upp_d: &Vec<f32>, rhs: &mut Vec<f32>) -> Vec<f32>  {
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
    let mut result: Vec<f32> = vec![];
    result.push(rhs[dim-1] / main_d[dim-1]);
    for i in 1..dim{
        result.push((rhs[dim-1-i] - upp_d[dim-1-i] * result[i-1]) / main_d[dim-1-i]);
    }
    result.reverse();
    result
}

impl Surface1D {
    fn get_slope(&mut self, dem: &Dem1D) -> &Self {
        self.slope = Some(disp::slope1d(&dem.x, &self.z));
        self
    }

    pub fn from_slbl(config: &SlideConfig, dem: &Dem1D) -> Self {
        let z_slbl = compute_slide(config, dem);
        Surface1D::new(z_slbl)
    }
}

impl DispProfile {
    pub fn from_surface(surface: &mut Surface1D, dem: &Dem1D, first_x: usize, last_x: usize) -> Self {
        match surface.slope {
            None => {surface.get_slope(dem); ()},
            _ => (),
        };
        let slope = surface.slope.clone().unwrap();
        let len = slope.len();
        let origin = pilar_slope(first_x, last_x, &surface.z, &slope, &dem.x, &dem.surface.z);
        DispProfile::new(slope, vec![-1.; len], origin.0, origin.1)
    }
}

#[cfg(test)]
mod tests {

}