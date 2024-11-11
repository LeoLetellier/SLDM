use crate::{prelude::Vector2Rep, types::*};
use std::f32::consts::PI;
use assert_approx_eq::assert_approx_eq;


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
        SlideMethod::RoutineThreshold => todo!(),
        SlideMethod::ExactMatrix => slbl_matrix(dem, config),
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

pub fn slbl_matrix2(dem: &Dem1D, first_pnt: usize, last_pnt: usize, tol: f32) -> Vec<f32> {
    let dim: usize = last_pnt - first_pnt - 1;
    let sub_diag: Vec<f32> = vec![-0.5 ; dim-1];
    let mut main_diag: Vec<f32> = vec![1. ; dim];
    let mut rhs: Vec<f32> = vec![-tol ; dim];
    rhs[0] += dem.surface.z[first_pnt] / 2.;
    rhs[dim-1] += dem.surface.z[last_pnt] / 2.;
    let m_result = tridiag_matrix_non_conservative(dim, &sub_diag, &mut main_diag, &sub_diag, &mut rhs);
    let mut result = dem.surface.z.to_owned();
    for i in (first_pnt + 1)..last_pnt {
        result[i] = m_result[i - first_pnt - 1]
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

fn slbl_routine_simple2(dem: &Dem1D, first_pnt: usize, last_pnt: usize, tol: f32, n_it: usize) -> Vec<f32> {
    let mut m_result = dem.surface.z[first_pnt..=last_pnt].to_owned();
    let mut result = dem.surface.z.to_owned();

    for _ in 0..n_it {
        let z_temp = m_result.clone();
        for i in 1..m_result.len()-1 {
            let local_mean = (z_temp[i-1] + z_temp[i+1]) / 2. - tol;
            if local_mean < m_result[i] {
                m_result[i] = local_mean;
            }
        }
    }

    for i in (first_pnt + 1)..last_pnt {
        result[i] = m_result[i - first_pnt + 1]
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
    pub(crate) fn get_slope(&mut self, dem: &Dem1D) -> &Self {
        self.slope = Some(slope1d(&dem.x, &self.z));
        self
    }

    pub fn from_slbl(config: &SlideConfig, dem: &Dem1D) -> Self {
        let z_slbl = compute_slide(config, dem);
        Surface1D::new(z_slbl)
    }

    pub fn from_slbl_exact(dem: &Dem1D, first_pnt: usize, last_pnt: usize, tol: f32) -> Self {
        let z_slbl = slbl_matrix2(dem, first_pnt, last_pnt, tol);
        Surface1D::new(z_slbl)
    }

    pub fn from_min_surf(surf1: &Surface1D, surf2: &Surface1D) -> Surface1D {
        let mut new_z = Vec::with_capacity(surf1.z.len());
        for k in 0..surf1.z.len() {
            if surf1.z[k] < surf2.z[k] {
                new_z.push(surf1.z[k]);
            } else {
                new_z.push(surf2.z[k]);
            }
        }
        Surface1D::new(new_z)
    }

    pub fn from_max_surf(surf1: &Surface1D, surf2: &Surface1D) -> Surface1D {
        let mut new_z = Vec::with_capacity(surf1.z.len());
        for k in 0..surf1.z.len() {
            if surf1.z[k] > surf2.z[k] {
                new_z.push(surf1.z[k]);
            } else {
                new_z.push(surf2.z[k]);
            }
        }
        Surface1D::new(new_z)
    }
}

fn find_nearest_abscissa(xs: &Vec<f32>, x: f32) -> usize {
    let mut left = 0;
    let mut right = xs.len() - 1;
    assert!((xs.len() > 2) & (xs.is_sorted()));
    assert!((xs[0] <= x) & (xs[xs.len() - 1] >= x));

    while left < right {
        let mid = (left + right) / 2;
        if xs[mid] <= x {
            left = mid + 1;
        } else {
            right = mid;
        }
        println!("{left}, {right}");
    }
    let dleft = x - xs[left - 1];
    let dright = xs[left] - x;
    println!("indexes {}, {}, values {}, {}", left - 1, left, xs[left - 1], xs[left]);
    println!("{dleft}, {dright}");
    if dleft <= dright {
        left - 1
    } else {
        left
    }
}

/// Computes the slope of a property along the section and the given DEM
pub(super) fn slope1d(x: &Vec<f32>, z: &Vec<f32>) -> Vec<f32> {
    assert_eq!(x.len(), z.len());
    let len = x.len();
    let mut slope_v: Vec<f32> = vec![];
    slope_v.push(((z[1] - z[0]) / (x[1] - x[0])).atan()); // half slope
    for i in 1..(len - 1) {
        slope_v.push(((z[i+1] - z[i-1]) / (x[i+1] - x[i-1])).atan()); // rad
    }
    slope_v.push(((z[len - 1] - z[len - 2]) / (x[len - 1] - x[len - 2])).atan()); // half slope
    slope_v
}

#[derive(Debug)]
pub enum SlopeError {
    InconsistentLen,
    VecTooSmall,
}

pub fn slope_asvec2(x: &Vec<f32>, z: &Vec<f32>) -> Result<Vec<Vector2Rep>, SlopeError> {
    if x.len() != z.len() {
        Err(SlopeError::InconsistentLen)
    } else if x.len() < 3 {
        Err(SlopeError::VecTooSmall)
    } else {
        let mut slope_vecs = vec![];
        slope_vecs.push(Vector2Rep::new(x[1] - x[0], z[1] - z[0]));
        for k in 1..(x.len() - 1) {
            slope_vecs.push(Vector2Rep::new(x[k + 1] - x[k - 1], z[k + 1] - z[k - 1]));
        }
        slope_vecs.push(Vector2Rep::new(x[x.len() - 1] - x[x.len() - 2], z[x.len() - 1] - z[x.len() - 2]));
        Ok(slope_vecs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_absissa() {
        let xs: Vec<f32> = vec![1., 2., 3., 4., 5., 6.];

        let result = find_nearest_abscissa(&xs, 2.29);
        assert_eq!(result, 1);

        let result = find_nearest_abscissa(&xs, 2.56);
        assert_eq!(result, 2);

        let result = find_nearest_abscissa(&xs, 1.);
        assert_eq!(result, 0);

        let result = find_nearest_abscissa(&xs, 3.5);
        assert_eq!(result, 2);
    }

    #[test]
    #[should_panic]
    fn test_absissa_fail () {
        let xs: Vec<f32> = vec![1., 2., 3., 4., 5., 6.];

        let result = find_nearest_abscissa(&xs, 12.);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_slope_asvec2() {
        let x = vec![0., 1., 2., 3., 4., 5.];
        let z = vec![6., 4., 2., 2., 6., 7.];
        let slope_vecs = slope_asvec2(&x, &z).unwrap();
        let result: Vec<f32> = slope_vecs.iter().map(|v| v.angle_rad()).collect();
        let expect: Vec<f32> = vec![(-2.0_f32/1.).atan(), (-4.0_f32/2.).atan(), (-2.0_f32/2.).atan(), (4.0_f32/2.).atan(), (5.0_f32/2.).atan(), (1.0_f32/1.).atan()];
        for k in 0..result.len() {
            assert_approx_eq!(result[k], expect[k]);
        }
    }
}