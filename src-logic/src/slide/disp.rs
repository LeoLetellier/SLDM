//! Additionnal functions to compute some slope and segment intersection

use crate::prelude::{self, PropOnSection};
use std::f64::consts::PI;

use super::*;

/// Computes the slope of a property along the section and the given DEM
pub(super) fn slope(dem: &prelude::Dem1D, z: &PropOnSection) -> PropOnSection {
    assert_eq!(dem.len, z.prop.len());
    let mut slope_v: Vec<f64> = vec![];
    slope_v.push(((z.prop[1] - z.prop[0]) / (dem.x[1] - dem.x[0])).atan()); // half slope
    for i in 1..(dem.len - 1) {
        slope_v.push(((z.prop[i+1] - z.prop[i-1]) / (dem.x[i+1] - dem.x[i-1])).atan()); // rad
    }
    slope_v.push(((z.prop[dem.len-1] - z.prop[dem.len-2]) / (dem.x[dem.len-1] - dem.x[dem.len-2])).atan()); // half slope
    let name = String::from("slope");
    PropOnSection::new(name, slope_v)
}

/// Computes the displacement projected from the failure surface into the topography (DEM) perpendicularly 
/// to the slope of the failure surface
pub(super) fn pilar_slope(config: &SlideConfig, surface: &PropOnSection, slope: &PropOnSection, dem: &prelude::Dem1D) -> (PropOnSection, PropOnSection) {
    let mut pilarx_v = dem.x.clone();
    let mut pilarz_v = dem.z.clone();

    for k in (config.first_pnt+1)..config.last_pnt {
        let coeff_dir = slope.prop[k].to_owned();
        let coeff_dir = match coeff_dir { // convert slope to perpendicular slope
            a if a >= 0. => a - PI/2.,
            a if a < 0. => a + PI/2.,
            _ => panic!("Not expecting a slope between pi/2 and 3pi/2 in pilar_slope."),
        };
        let xx: (f64, f64) = (dem.x.first().unwrap().to_owned(), dem.x.last().unwrap().to_owned());
        let zz: (f64, f64) = (surface.prop[k] + coeff_dir * (xx.0 - dem.x[k]), surface.prop[k] + coeff_dir * (xx.1 - dem.x[k]));
        let intercept = intersection_on_topo(dem, xx, zz);
        if intercept.is_none() {
            panic!("No intersection found in pilar_slope at point {}", k);
        }
        pilarx_v[k] = intercept.unwrap().0;
        pilarz_v[k] = intercept.unwrap().1;
    }
    (PropOnSection::new(String::from("pilarx"), pilarx_v), PropOnSection::new(String::from("pilary"), pilarz_v))
}

/// Compute the intersection between two segments, if exist
fn get_intersection_point(xk: (f64, f64), zk: (f64, f64), xx: (f64, f64), zz: (f64, f64)) -> Option<(f64, f64)> {
    let (x1, y1, x2, y2): (f64, f64, f64, f64) = (xk.0, zk.0, xk.1, zk.1);
    let (x3, y3, x4, y4): (f64, f64, f64, f64) = (xx.0, zz.0, xx.1, zz.1);
    
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
fn intersection_on_topo(dem: &prelude::Dem1D, xx :(f64, f64), zz: (f64, f64)) -> Option<(f64, f64)> {
    assert_eq!(dem.x.len(), dem.z.len());
    for k in 1..dem.x.len() {
        // got x overlap
        if xx.0 <= dem.x[k] && dem.x[k-1] <= xx.1 {
            let intersect = get_intersection_point((dem.x[k-1], dem.x[k]), (dem.z[k-1], dem.z[k]), xx, zz);
            if intersect.is_some() {
                return intersect
            }
        }
    }
    None
}

/// Converts a slope (rad) to a 2D vector (x-z local coordinates) 
pub(super) fn slope2vec(slope: &PropOnSection, dir: i8, first_index: usize, last_index: usize) -> (PropOnSection, PropOnSection) {
    let mut vec_x: Vec<f64> = slope.prop.clone().iter().map(|x| x.cos() * dir as f64).collect();
    let mut vec_z: Vec<f64> = slope.prop.clone().iter().map(|z| z.sin() * dir as f64).collect();
    for k in 0..vec_x.len() {
        if k < first_index || k > last_index {
            vec_x[k] = 0.;
            vec_z[k] = 0.;
        }
    }
    (PropOnSection::new(String::from("vecx"), vec_x), PropOnSection::new(String::from("vecy"), vec_z))
}


//test crossing TODO

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::Dem1D;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_slope() {
        let x = vec![0., 100., 200., 300., 400., 500., 600.];
        let z = vec![0., 10., 30., 35., 45., 50., 60.];
        let prop = vec![0., 10., 14., 22., 34., 50., 60.];
        let dem = Dem1D::new(x, z);

        let prop_sec = PropOnSection::new(String::from("test"), prop);
        
        let res = slope(&dem, &prop_sec).prop;
        let expect = [0.0996687, 0.069886, 0.0599282, 0.0996687, 0.1390959, 0.129275, 0.0996687];
        for i in 0..res.len() {
            assert_approx_eq!(res[i], expect[i]);
        }
    }

    #[test]
    fn test_intercept() {
        let res = get_intersection_point((1., 2.), (1., 2.), (1., 2.), (2., 1.));
        let expect = Some((1.5, 1.5));
        assert_eq!(res, expect);
    }
}
