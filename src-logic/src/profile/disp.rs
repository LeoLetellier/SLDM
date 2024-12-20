//! Additionnal functions to compute some slope and segment intersection

use super::interpol_linear;
use std::f32::consts::PI;

#[derive(Debug)]
pub enum PillarError {
    InvalidSlope,
    NoIntersection,
}

/// Computes the displacement projected from the failure surface into the topography (DEM) perpendicularly
/// to the slope of the failure surface
pub fn pillar_slope(
    first_x: usize,
    last_x: usize,
    slide_z: &Vec<f32>,
    slope: &Vec<f32>,
    x: &Vec<f32>,
    z: &Vec<f32>,
) -> Result<(Vec<f32>, Vec<f32>), PillarError> {
    let mut ground_proj_x = x.clone();
    let mut ground_proj_z = z.clone();

    for k in (first_x + 1)..last_x {
        if (slide_z[k] - z[k]).abs() > 1e-6_f32 {
            println!("enter in the matter");
            let coeff_dir = slope[k].to_owned();
            let coeff_dir = match coeff_dir {
                // convert slope to perpendicular slope
                a if a >= 0. => a - PI / 2.,
                a if a < 0. => a + PI / 2.,
                _ => {
                    return Err(PillarError::InvalidSlope);
                }
            };
            let xx: (f32, f32) = (x.first().unwrap().to_owned(), x.last().unwrap().to_owned());
            let zz: (f32, f32) = (
                slide_z[k] + coeff_dir.tan() * (xx.0 - x[k]),
                slide_z[k] + coeff_dir.tan() * (xx.1 - x[k]),
            );
            let intercept = intersection_on_topo(x, z, xx, zz);
            let intercept = match intercept {
                Some(i) => i,
                None => {
                    return Err(PillarError::NoIntersection);
                }
            };
            ground_proj_x[k] = intercept.0;
            ground_proj_z[k] = intercept.1;
        }
    }
    Ok((ground_proj_x, ground_proj_z))
}

/// Compute the intersection between two segments, if exist
fn get_intersection_point(
    xk: (f32, f32),
    zk: (f32, f32),
    xx: (f32, f32),
    zz: (f32, f32),
) -> Option<(f32, f32)> {
    let (x1, y1, x2, y2): (f32, f32, f32, f32) = (xk.0, zk.0, xk.1, zk.1);
    let (x3, y3, x4, y4): (f32, f32, f32, f32) = (xx.0, zz.0, xx.1, zz.1);

    let denominator = (x1 - x2) * (y3 - y4) - (x3 - x4) * (y1 - y2);
    assert_ne!(0., denominator, "tested segments are parallels");
    let intersection_x =
        ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denominator;
    let intersection_y =
        ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denominator;
    match intersection_x {
        x if (x > xk.0 && x < xk.1) => Some((intersection_x, intersection_y)),
        _ => None,
    }
}

/// Compute the intersection between a segment and a set of segment, if exist
fn intersection_on_topo(
    x: &Vec<f32>,
    z: &Vec<f32>,
    xx: (f32, f32),
    zz: (f32, f32),
) -> Option<(f32, f32)> {
    assert_eq!(x.len(), z.len());
    for k in 1..x.len() {
        // got x overlap
        if xx.0 <= x[k] && x[k - 1] <= xx.1 {
            let intersect = get_intersection_point((x[k - 1], x[k]), (z[k - 1], z[k]), xx, zz);
            if intersect.is_some() {
                return intersect;
            }
        }
    }
    None
}

pub(super) fn amplitude_gradient(
    target: &Vec<f32>,
    gradient_weights: &Vec<(usize, f32)>,
) -> Vec<f32> {
    let gradient_vector = interpol_linear(
        &gradient_weights.iter().map(|(a, _)| *a as f32).collect(),
        &gradient_weights.iter().map(|(_, b)| *b).collect(),
        &(0..target.len()).map(|i| i as f32).collect(),
    );
    (0..target.len())
        .map(|k| target[k] * gradient_vector[k])
        .collect()
}

//test crossing TODO

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercept() {
        let res = get_intersection_point((1., 2.), (1., 2.), (1., 2.), (2., 1.));
        let expect = Some((1.5, 1.5));
        assert_eq!(res, expect);
    }

    #[test]
    fn test_gradient() {
        let profile: Vec<f32> = vec![1.; 12];
        let gradient = vec![(3, 2.), (5, 1.), (10, 1.5)];
        let result = amplitude_gradient(&profile, &gradient);
        let expect = vec![2., 2., 2., 2., 1.5, 1., 1.1, 1.2, 1.3, 1.4, 1.5, 1.5];
        assert_eq!(result, expect);
    }
}
