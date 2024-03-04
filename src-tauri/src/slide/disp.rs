use crate::types::{self, PropOnSection};
use std::f64::consts::PI;

use super::*;


pub(super) fn slope(dem: &types::Dem, z: &PropOnSection) -> PropOnSection {
    assert_eq!(dem.len, z.prop.len());
    let mut slope_v: Vec<f64> = vec![];
    slope_v.push(((z.prop[1] - z.prop[0]) / (dem.x[1] - dem.x[0])).atan()); // half slope
    slope_v.push(((z.prop[2] - z.prop[0]) / (dem.x[2] - dem.x[0])).atan());
    for i in 2..(dem.len - 1) {
        slope_v.push(((z.prop[i+1] - z.prop[i-1]) / (dem.x[i+1] - dem.x[i-1])).atan()); // rad
    }
    slope_v.push(((z.prop[dem.len-1] - z.prop[dem.len-2]) / (dem.x[dem.len-1] - dem.x[dem.len-2])).atan()); // half slope
    let name = String::from("slope");
    PropOnSection::new(name, slope_v, types::Unit::Degree)
}

pub(super) fn pilar_slope(config: &SlideConfig, surface: &PropOnSection, slope: &PropOnSection, dem: &types::Dem) -> (PropOnSection, PropOnSection) {
    let mut pilarx_v = dem.x.clone();
    let mut pilarz_v = dem.z.clone();

    for k in (config.first_pnt+1)..config.last_pnt {
        let coeff_dir = slope.prop[k].to_owned();
        let coeff_dir = match coeff_dir { // convert slope to perpendicular slope
            a if a > 0. && a <= PI/2. => a + 3.*PI/2.,
            a if a <= 0. && a >= 3.*PI/2. => a - 3.*PI/2.,
            _ => panic!("Not expecting a slope between pi/2 and 3pi/2 in pilar_slope."),
        };
        let intercept = intersection_on_topo(dem, (dem.x[k], dem.x[k] + 1.), (surface.prop[k], surface.prop[k] + 1.));
        if intercept == None {
            panic!("No intersection found in pilar_slope at point {}", k);
        }
        pilarx_v.push(intercept.unwrap().0);
        pilarz_v.push(intercept.unwrap().1);
    }
    (PropOnSection::new(String::from("pilarx"), pilarx_v, types::Unit::Meter), PropOnSection::new(String::from("pilary"), pilarz_v, types::Unit::Meter))
}

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


fn intersection_on_topo(dem: &types::Dem, xx :(f64, f64), zz: (f64, f64)) -> Option<(f64, f64)> {
    assert_eq!(dem.x.len(), dem.z.len());
    for k in 1..dem.x.len() {
        if xx.0 <= dem.x[k] && dem.x[k-1] <= xx.1 {
            let intersect = get_intersection_point((dem.x[k-1], dem.x[k]), (dem.z[k-1], dem.z[k]), xx, zz);
            if intersect != None {
                return intersect
            }
        }
    }
    None
}

// convert a slope (rad) to a 2D vector (x-z coordinates) 
pub(super) fn slope2vec(slope: &PropOnSection) -> (PropOnSection, PropOnSection) {
    let mut vec_x: Vec<f64> = slope.prop.clone().iter().map(|x| x.cos()).collect();
    let mut vec_z: Vec<f64> = slope.prop.clone().iter().map(|z| z.sin()).collect();
    (PropOnSection::new(String::from("vecx"), vec_x, types::Unit::Meter), PropOnSection::new(String::from("vecy"), vec_z, types::Unit::Meter))
}


//test crossing TODO
