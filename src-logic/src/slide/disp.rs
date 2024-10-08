//! Additionnal functions to compute some slope and segment intersection

use std::f32::consts::PI;

use assert_approx_eq::assert_approx_eq;
use log::debug;

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

/// Computes the displacement projected from the failure surface into the topography (DEM) perpendicularly 
/// to the slope of the failure surface
pub(super) fn pillar_slope(first_x: usize, last_x: usize, slide_z: &Vec<f32>, slope: &Vec<f32>, x: &Vec<f32>, z: &Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    let mut ground_proj_x = x.clone();
    let mut ground_proj_z = z.clone();

    for k in (first_x + 1)..last_x {
        let coeff_dir = slope[k].to_owned();
        let coeff_dir = match coeff_dir { // convert slope to perpendicular slope
            a if a >= 0. => a - PI/2.,
            a if a < 0. => a + PI/2.,
            _ => panic!("Not expecting a slope between pi/2 and 3pi/2 in pilar_slope."),
        };
        let xx: (f32, f32) = (x.first().unwrap().to_owned(), x.last().unwrap().to_owned());
        let zz: (f32, f32) = (slide_z[k] + coeff_dir.tan() * (xx.0 - x[k]), slide_z[k] + coeff_dir.tan() * (xx.1 - x[k]));
        let intercept = intersection_on_topo(x, z, xx, zz);
        if intercept.is_none() {
            panic!("No intersection found in pilar_slope at point {}", k);
        }
        ground_proj_x[k] = intercept.unwrap().0;
        ground_proj_z[k] = intercept.unwrap().1;
    }
    (ground_proj_x, ground_proj_z)
}

/// Compute the intersection between two segments, if exist
fn get_intersection_point(xk: (f32, f32), zk: (f32, f32), xx: (f32, f32), zz: (f32, f32)) -> Option<(f32, f32)> {
    let (x1, y1, x2, y2): (f32, f32, f32, f32) = (xk.0, zk.0, xk.1, zk.1);
    let (x3, y3, x4, y4): (f32, f32, f32, f32) = (xx.0, zz.0, xx.1, zz.1);
    
    let denominator = (x1 - x2) * (y3 - y4) - (x3 - x4) * (y1 - y2);
    assert_ne!(0., denominator, "tested segments are parallels");
    let intersection_x = ((x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4)) / denominator;
    let intersection_y = ((x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4)) / denominator;
    match intersection_x {
        x if (x > xk.0 && x < xk.1) => Some((intersection_x, intersection_y)),
        _ => None,
    }
}

/// Compute the intersection between a segment and a set of segment, if exist
fn intersection_on_topo(x: &Vec<f32>, z: &Vec<f32>, xx :(f32, f32), zz: (f32, f32)) -> Option<(f32, f32)> {
    assert_eq!(x.len(), z.len());
    for k in 1..x.len() {
        // got x overlap
        if xx.0 <= x[k] && x[k-1] <= xx.1 {
            let intersect = get_intersection_point((x[k-1], x[k]), (z[k-1], z[k]), xx, zz);
            if intersect.is_some() {
                return intersect
            }
        }
    }
    None
}

/// Converts a slope (rad) to a 2D vector (x-z local coordinates) 
pub(super) fn slope2vec(slope: &Vec<f32>, dir: i8, first_index: usize, last_index: usize) -> (Vec<f32>, Vec<f32>) {
    let mut vec_x: Vec<f32> = slope.clone().iter().map(|x| x.cos() * dir as f32).collect();
    let mut vec_z: Vec<f32> = slope.clone().iter().map(|z| z.sin() * dir as f32).collect();
    for k in 0..vec_x.len() {
        if k < first_index || k > last_index {
            vec_x[k] = 0.;
            vec_z[k] = 0.;
        }
    }
    (vec_x, vec_z)
}


//test crossing TODO

#[cfg(test)]
mod tests {
    use super::*;
    // use crate::prelude::*;
    // use assert_approx_eq::assert_approx_eq;

    // #[test]
    // fn test_slope() {
    //     let x = vec![0., 100., 200., 300., 400., 500., 600.];
    //     let z = vec![0., 10., 30., 35., 45., 50., 60.];
    //     let prop = vec![0., 10., 14., 22., 34., 50., 60.];
    //     let dem = Dem1D::new(x, z);

    //     let prop_sec = PropOnSection::new(String::from("test"), prop);
        
    //     let res = slope(&dem, &prop_sec).prop;
    //     let expect = [0.0996687, 0.069886, 0.0599282, 0.0996687, 0.1390959, 0.129275, 0.0996687];
    //     for i in 0..res.len() {
    //         assert_approx_eq!(res[i], expect[i]);
    //     }
    // }

    #[test]
    fn test_intercept() {
        let res = get_intersection_point((1., 2.), (1., 2.), (1., 2.), (2., 1.));
        let expect = Some((1.5, 1.5));
        assert_eq!(res, expect);
    }
}
