//! Compute a displacement for a landslide based on geometric assumptions.
//! 
//! Considering that the displacement in the section is propagating along the failure surface, this 
//! displacement is estimated as unit vectors along this slope. Multiple failure surfaces can be combined
//! to contribute to a global model. The amplitude of the model can be dimensionned by user input or
//! extern displacement data located on the section.

use std::sync::Arc;
use std::f64::consts::PI;
use nnls::nnls;
use ndarray::prelude::*;

use crate::types::{PropOnSection, Orientation};
use crate::slide::Slide;

/// Parameters and variable of a displacement model
#[derive(Debug)]
pub struct DispModel {
    mdl_vec: (PropOnSection, PropOnSection),
    vec_slope: PropOnSection,
    main_slide: Arc<Slide>,
    mdl_los_disp: Option<PropOnSection>,
    ground_perspective: bool,
    sar_displacement: Option<PropOnSection>,
}

impl DispModel {
    /// Build a model given a list of slides and their respective contribution (ratio)
    pub(crate) fn build(slide_list: &Vec<Arc<Slide>>, main_slide: Arc<Slide>, len: usize) -> DispModel {
        let mut vec_x = vec![0.; len];
        let mut vec_z = vec![0.; len];
        let mut vec_slope = vec![0.; len];
        // Sum all displacements at each point, weighted by their ratios
        for slide in slide_list {
            let vector = slide.vector_slope.as_ref().unwrap();
            let ratio = slide.ratio;
            for k in 0..len {
                vec_x[k] += vector.0.prop[k] * ratio;
                vec_z[k] += vector.1.prop[k] * ratio;
            }
        }

        // Computing the associated slope and incidence angle for the new displacement vectors
        for k in 0..len {
            vec_slope[k] = match vec_x[k] {
                i if i != 0. => (vec_z[k] / vec_x[k].abs()).atan() * slide_list.len() as f64,
                _ => 0.,
            };
        }
        
        let mdl_vec = (
            PropOnSection::new(String::from("mdl_vec_x"),vec_x),
            PropOnSection::new(String::from("mdl_vec_z"),vec_z)
            );

        let slope_prop = PropOnSection::new(String::from("model slope"), vec_slope);
             
        DispModel {
            mdl_vec,
            vec_slope: slope_prop,
            main_slide: Arc::clone(&main_slide),
            mdl_los_disp: None,
            ground_perspective: true, // inverse the sign so that ground going down corresponds to negative displacement
            sar_displacement: None,
        }
    }

    /// Compute the displacement previously computed by the model in a Line Of Sight (LOS)
    pub fn compute_mdl_disp_in_los(&mut self, section_orientation: &Orientation, sar_geometry: &Orientation) {
        let mut disp_in_los = vec![0.; self.vec_slope.prop.len()];
        for k in 0..self.vec_slope.prop.len() {
            let slope = self.vec_slope.prop[k];
            // If the vector is going towards negative x-axis, the azimuth should be corrected
            let dir = match self.mdl_vec.0.prop[k] {
                i if i >= 0. => true,
                _ => false,
            };
            let amplitude = (self.mdl_vec.0.prop[k] * self.mdl_vec.0.prop[k] + self.mdl_vec.1.prop[k] * self.mdl_vec.1.prop[k]).sqrt();
            // projection of SEC (displacement in the section) into the LOS is 
            // Amplitude of displacement * inner_product(unit SEC vector, unit LOS vector)
            disp_in_los[k] = amplitude * local_projection(slope, dir, &section_orientation, &sar_geometry);
            // The calculated projection is set by the LOS (facing down)
            if self.ground_perspective{
                disp_in_los[k] *= -1.; // to match sign convention, considering from the ground perspective and not the satellite
            }
        }
        self.mdl_los_disp = Some(PropOnSection::new(String::from("disp_in_los"), disp_in_los));
    }

    pub(crate) fn build_and_fit() {
        
    }
}

/// Performs the local projection of two unit vectors, simplify into inner product
fn local_projection(local_slope: f64, dir_az: bool, section_orientation: &Orientation, sar_geometry: &Orientation) -> f64 {
    // if the vector is not oriented in the same direction as the section axis x, it is not oriented as azimuth but rather -azimuth
    // thus parameter dir is set to false to set azimuth to -azimuth
    let sec_az = match dir_az {
        true => section_orientation.azimuth,
        _ => section_orientation.azimuth + PI,
    };
    let sec_in = match local_slope {
        i if i >= 0. => PI / 2. + i.abs(),
        i => PI / 2. - i.abs(),
    };
    let vec_section = geometry3d_to_vector3d(sec_az, sec_in);

    let sar_in = sar_geometry.incidence;
    let sar_az = sar_geometry.azimuth;
    let vec_sar = geometry3d_to_vector3d(sar_az, sar_in);
    
    inner_product_3(vec_section, vec_sar)
}

/// Translate 3D geometry into 3D vectors before performing the inner product
fn geometry3d_to_vector3d(azimuth: f64, incidence: f64) -> [f64; 3] {
    [azimuth.cos() * incidence.sin(), azimuth.sin() * incidence.sin(), -incidence.cos()]
}

fn fit_ratios(slide_list: &Vec<Arc<Slide>>, sar_disp: &Vec<PropOnSection>) {
    let n_slide = slide_list.len();
    let n_points = sar_disp[0].prop.len();
    let a: Array<f64, _> = Array2::zeros((n_slide, n_points));
    let b: Array<f64, _> = Array1::zeros(n_points);
    for s in 0..n_slide {
        for p in 0..n_points {
            // a[s, p] = slide_list[s].
            // need also prjecting here 
        }
    }
}

/// Performs the inner product between two 3d vectors
fn inner_product_3(x: [f64; 3], y: [f64; 3]) -> f64 {
    x[0] * y[0] + x[1] * y[1] + x[2] * y[2]
}

fn rmse(x: &Vec<f64>) -> f64 {
    let res = x.iter().fold(0., |acc, el| acc + el * el / x.len() as f64).sqrt();
    res
}


#[cfg(test)]
mod test{
    use crate::types::Orientation;
    use super::local_projection;

    #[test]
    fn test_local_proj(){
        let incidence = 1.56;
        let dir = false;
        let section = Orientation::new(1.919, 0.);
        let sar = Orientation::new(4.974, 0.610);
        let prod = local_projection(incidence, dir, &section, &sar);
        dbg!(prod);
    }
}