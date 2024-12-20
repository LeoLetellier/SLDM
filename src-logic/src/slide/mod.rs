use crate::{
    prelude::{rad2deg, Vector2Rep},
    types::*,
};
#[allow(unused_imports)] // actually used but raises unused import
use assert_approx_eq::assert_approx_eq;

pub fn slbl_matrix(dem: &Dem1D, first_pnt: usize, last_pnt: usize, tol: f32) -> Vec<f32> {
    let dim: usize = last_pnt - first_pnt - 1;
    let sub_diag: Vec<f32> = vec![-0.5; dim - 1];
    let mut main_diag: Vec<f32> = vec![1.; dim];
    let mut rhs: Vec<f32> = vec![-tol; dim];
    rhs[0] += dem.surface.z[first_pnt] / 2.;
    rhs[dim - 1] += dem.surface.z[last_pnt] / 2.;
    let m_result =
        tridiag_matrix_non_conservative(dim, &sub_diag, &mut main_diag, &sub_diag, &mut rhs);
    let mut result = dem.surface.z.to_owned();
    for i in (first_pnt + 1)..last_pnt {
        result[i] = m_result[i - first_pnt - 1]
    }
    result
}

#[allow(dead_code)]
fn slbl_routine_simple(
    dem: &Dem1D,
    first_pnt: usize,
    last_pnt: usize,
    tol: f32,
    n_it: usize,
) -> Vec<f32> {
    let mut m_result = dem.surface.z[first_pnt..=last_pnt].to_owned();
    let mut result = dem.surface.z.to_owned();

    for _ in 0..n_it {
        let z_temp = m_result.clone();
        for i in 1..m_result.len() - 1 {
            let local_mean = (z_temp[i - 1] + z_temp[i + 1]) / 2. - tol;
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
fn slbl_routine_thresholds(
    z_topo: &Vec<f32>,
    n_it: usize,
    tol: f32,
    elevation_min: Option<f32>,
    slope_max: Option<(f32, f32)>,
) -> (Vec<f32>, usize) {
    let mut z_slbl = z_topo.to_owned();
    let mut current_it: usize = 0;
    'routine: while current_it < n_it {
        let z_temp = z_slbl.clone();
        '_section: for i in 1..(z_slbl.len() - 1) {
            let local_mean = (z_temp[i - 1] + z_temp[i + 1]) / 2. - tol;
            if let Some(elevation_min) = elevation_min {
                if local_mean < elevation_min {
                    break 'routine; // vs continue 'section
                }
            }
            if let Some((slope_max, x_spacing)) = slope_max {
                // not only left slope
                if (
                    local_slope_deg(z_slbl[i - 1], local_mean, x_spacing).abs(),
                    local_slope_deg(local_mean, z_slbl[i + 1], x_spacing).abs(),
                ) > (slope_max.abs(), slope_max.abs())
                {
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
    let local_vec = Vector2Rep::new(spacing, second_point - first_point);
    local_vec.angle_rad()
}

fn local_slope_deg(first_point: f32, second_point: f32, spacing: f32) -> f32 {
    let local_slope_rad = local_slope(first_point, second_point, spacing);
    rad2deg(local_slope_rad)
}

/// Solver of linear system A.X=B where A is tridiagonal
fn tridiag_matrix_non_conservative(
    dim: usize,
    low_d: &Vec<f32>,
    main_d: &mut Vec<f32>,
    upp_d: &Vec<f32>,
    rhs: &mut Vec<f32>,
) -> Vec<f32> {
    // Check that the sizes are matching
    assert_eq!(
        low_d.len(),
        dim - 1,
        "Low_d doesn't match the problem's dimension."
    );
    assert_eq!(
        upp_d.len(),
        dim - 1,
        "Upp_d doesn't match the problem's dimension."
    );
    assert_eq!(
        main_d.len(),
        dim,
        "Main_d doesn't match the problem's dimension."
    );
    assert_eq!(rhs.len(), dim, "Rhs doesn't match the problem's dimension.");
    // Update coefficient for computation
    // Initial values for main_d and rhs are not conserved
    for i in 1..dim {
        let temp_value = low_d[i - 1] / main_d[i - 1];
        main_d[i] -= temp_value * upp_d[i - 1];
        rhs[i] -= temp_value * rhs[i - 1];
    }
    // Back substitution to obtain the result
    let mut result: Vec<f32> = vec![];
    result.push(rhs[dim - 1] / main_d[dim - 1]);
    for i in 1..dim {
        result.push((rhs[dim - 1 - i] - upp_d[dim - 1 - i] * result[i - 1]) / main_d[dim - 1 - i]);
    }
    result.reverse();
    result
}

impl Surface1D {
    pub fn get_slope(&mut self, dem: &Dem1D) -> &Self {
        self.slope = Some(slope1d(&dem.x, &self.z));
        self
    }

    // pub fn from_slbl(config: &SlideConfig, dem: &Dem1D) -> Self {
    //     let z_slbl = compute_slide(config, dem);
    //     Surface1D::new(z_slbl)
    // }

    pub fn from_slbl_exact(dem: &Dem1D, first_pnt: usize, last_pnt: usize, tol: f32) -> Self {
        let z_slbl = slbl_matrix(dem, first_pnt, last_pnt, tol);
        Surface1D::new(z_slbl)
    }

    pub fn from_slbl_routine(
        dem: &Dem1D,
        first_pnt: usize,
        last_pnt: usize,
        tol: f32,
        n_it: usize,
        elevation_min: Option<f32>,
        slope_max: Option<f32>,
    ) -> Self {
        debug_assert!(first_pnt < last_pnt);
        debug_assert!(last_pnt <= dem.x.len());
        let x_spacing =
            dem.x.last().unwrap_or(&f32::default()) - dem.x.first().unwrap_or(&f32::default());
        let slope_max = match slope_max {
            Some(s) if x_spacing != f32::default() => Some((s, x_spacing)),
            _ => None,
        };
        let crop_dem = dem.surface.z[first_pnt..=last_pnt].to_owned();
        let (result_routine, _) =
            slbl_routine_thresholds(&crop_dem, n_it, tol, elevation_min, slope_max);
        let mut result_global = dem.surface.z.clone();
        result_global[first_pnt..=last_pnt]
            .copy_from_slice(&result_routine[0..=(last_pnt - first_pnt)]);

        Surface1D::new(result_global)
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

/// Computes the slope of a property along the section and the given DEM
pub(super) fn slope1d(x: &Vec<f32>, z: &Vec<f32>) -> Vec<f32> {
    assert_eq!(x.len(), z.len());
    let len = x.len();
    let mut slope_v: Vec<f32> = vec![];
    slope_v.push(((z[1] - z[0]) / (x[1] - x[0])).atan()); // half slope
    for i in 1..(len - 1) {
        slope_v.push(((z[i + 1] - z[i - 1]) / (x[i + 1] - x[i - 1])).atan()); // rad
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
        slope_vecs.push(Vector2Rep::new(
            x[x.len() - 1] - x[x.len() - 2],
            z[x.len() - 1] - z[x.len() - 2],
        ));
        Ok(slope_vecs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slope_asvec2() {
        let x = vec![0., 1., 2., 3., 4., 5.];
        let z = vec![6., 4., 2., 2., 6., 7.];
        let slope_vecs = slope_asvec2(&x, &z).unwrap();
        let result: Vec<f32> = slope_vecs.iter().map(|v| v.angle_rad()).collect();
        let expect: Vec<f32> = vec![
            (-2.0_f32 / 1.).atan(),
            (-4.0_f32 / 2.).atan(),
            (-2.0_f32 / 2.).atan(),
            (4.0_f32 / 2.).atan(),
            (5.0_f32 / 2.).atan(),
            (1.0_f32 / 1.).atan(),
        ];
        for k in 0..result.len() {
            assert_approx_eq!(result[k], expect[k]);
        }
    }
}
