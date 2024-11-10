
pub mod vec_proj;
use vec_proj::*;

use crate::types::*;

impl DispProfile {
    pub fn projected_amplitude_onto(&self, los: Orientation, section: Orientation) -> Vec<f32> {
        let los3: &Vector3Rep = &los.into();
        (0..self.vecs.len()).map(|k| {
            let vec3 = Vector3Rep::from_vertical_section_rad(&self.vecs[k], section.azimuth);
            vec3.inner_product(los3)
        }).collect()
    }
}


impl DispData {
    pub fn project_on_section(&mut self, section: Orientation) {
        todo!()
    }
}