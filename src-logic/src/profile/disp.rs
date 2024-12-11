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
    println!("slide elevation {:?}", slide_z);

    let xx: (f32, f32) = (x.first().unwrap().to_owned(), x.last().unwrap().to_owned());
    for k in (first_x + 1)..last_x {
        if (slide_z[k] - z[k]).abs() > 1e-6_f32 {
            let coeff_dir = slope[k].to_owned();
            let coeff_dir = match coeff_dir {
                // convert slope to perpendicular slope
                a if a >= 0. => a - PI / 2.,
                a => a + PI / 2.
            };
            let zz: (f32, f32) = (
                slide_z[k] + coeff_dir.tan() * (xx.0 - x[k]),
                slide_z[k] + coeff_dir.tan() * (xx.1 - x[k]),
            );
            let intercept = intersection_on_topo(x, z, xx, zz);
            let intercept = match intercept {
                Some(i) => i,
                None => {
                    log::error!("pillar slope: No intersection found (xx: {}, {}, zz: {}, {}), (cd {}, x0 {}, z0 {}, slope: {})",
                     xx.0, xx.1, zz.0, zz.1, coeff_dir, x[k], slide_z[k], slope[k]);
                    // return Err(PillarError::NoIntersection);
                    (x[k], slide_z[k])
                }
            };
            ground_proj_x[k] = intercept.0;
            ground_proj_z[k] = intercept.1;
        }
    }
    Ok((ground_proj_x, ground_proj_z))
}

/// Compute the intersection between two segments, if exist
/// 
/// [stackoverflow](https://stackoverflow.com/questions/563198/how-do-you-detect-where-two-line-segments-intersect)
fn get_intersection_point(
    (x1, x2): (f32, f32),
    (y1, y2): (f32, f32),
    (x3, x4): (f32, f32),
    (y3, y4): (f32, f32),
) -> Option<(f32, f32)> {
    let dx1 = x2 - x1;
    let dy1 = y2 - y1;
    let dx2 = x4 - x3;
    let dy2 = y4 - y3;
    
    let s = (-dy1 * (x1 - x3) + dx1 * (y1 - y3)) / (-dx2 * dy1 + dx1 * dy2);
    let t = ( dx2 * (y1 - y3) - dy2 * (x1 - x3)) / (-dx2 * dy1 + dx1 * dy2);

    if s < 0.0 && s > 1.0 && t < 0.0 && t > 1.0 {
        None
    } else {
        Some((x1 + t * dx1, y1 + (t * dy1)))
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
    fn test_intercept_on_point() {
        let res = get_intersection_point((1., 2.), (1., 2.), (1., 2.), (1., 3.));
        assert!(res.is_some());
        let expect = Some((1., 1.));
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
