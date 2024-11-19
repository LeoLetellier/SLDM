pub mod vec_proj;
use eqsolver::{global_optimisers::ParticleSwarm, SolverError};
use nalgebra::base::DVector;
use vec_proj::*;

use crate::types::*;

impl DispProfile {
    pub fn projected_amplitude_onto(&self, los: Orientation, section: Orientation) -> Vec<f32> {
        let los3: &Vector3Rep = &los.into();
        (0..self.vecs.len())
            .map(|k| {
                let vec3 = Vector3Rep::from_vertical_section_rad(&self.vecs[k], section.azimuth);
                vec3.inner_product(los3)
            })
            .collect()
    }
}

impl DispData {
    pub fn project_on_section(&mut self, _section: Orientation) {
        todo!()
    }
}

pub fn rmse(prediction: &[f32], observation: &[f32]) -> f32 {
    debug_assert_eq!(prediction.len(), observation.len());
    let len = prediction.len();
    (0..len)
        .fold(0.0, |acc, k| {
            acc + (prediction[k] - observation[k]) * (prediction[k] - observation[k]) / (len as f32)
        })
        .sqrt()
}

pub fn vec_to_na(vec: Vec<f32>) -> DVector<f32> {
    DVector::from_vec(vec)
}

pub fn na_to_vec(vec: DVector<f32>) -> Vec<f32> {
    vec.data.as_vec().to_vec()
}

#[derive(Debug)]
pub(crate) struct ComposedModel {
    dem: Dem1D,
    profiles_regul: Vec<DispProfile>,
    section_geometry: Orientation,
    los_geometry: Orientation,
    los_data: DispData,
}

impl ComposedModel {
    pub fn new(
        dem: &Dem1D,
        profiles_regul: &[DispProfile],
        section_geometry: &Orientation,
        los_geometry: &Orientation,
        los_data: &DispData,
    ) -> Self {
        ComposedModel {
            dem: dem.clone(),
            profiles_regul: profiles_regul.to_owned(),
            section_geometry: section_geometry.clone(),
            los_geometry: los_geometry.clone(),
            los_data: los_data.clone(),
        }
    }

    pub fn current_rmse(&self, disp_model: &Vec<f32>) -> f32 {
        rmse(disp_model, &self.los_data.amplitude)
    }

    pub fn compose_model(&self, weights: Vec<f32>) -> DispProfile {
        let nb_vecs = self.profiles_regul[0].vecs.len();
        let origins = self.profiles_regul[0].origins.to_owned();

        let mut sum_vx = vec![0.; nb_vecs];
        let mut sum_vz = vec![0.; nb_vecs];

        for k in 0..self.profiles_regul.len() {
            let mut profile = self.profiles_regul[k].to_owned();
            profile.weight_disp(weights[k]);
            for i in 0..nb_vecs {
                sum_vx[i] += profile.vecs[i].coords().0;
                sum_vz[i] += profile.vecs[i].coords().1;
            }
        }

        let vecs = (0..nb_vecs)
            .map(|k| Vector2Rep::new(sum_vx[k], sum_vz[k]))
            .collect();

        DispProfile::new(vecs, origins).unwrap()
    }

    pub fn objective_function(&self, vec: DVector<f32>) -> f32 {
        let mut profile = self.compose_model(na_to_vec(vec));
        let los_x = self.los_data.x.to_owned();
        let los_y = self.dem.interpolate_elevation_on_x(&los_x);
        let los_origins = &(0..los_x.len()).map(|k| [los_x[k], los_y[k]]).collect();
        profile.interpolate_on_origins(los_origins);
        let predicted = profile.projected_amplitude_onto(
            self.los_geometry.to_owned(),
            self.section_geometry.to_owned(),
        );

        self.current_rmse(&predicted)
    }

    pub fn fit_disp(&self) -> Result<Vec<f32>, SolverError> {
        // Profile weights can range from 0. to 1000.
        let lower_bounds = DVector::repeat(self.profiles_regul.len(), 0.0_f32);
        let upper_bounds = DVector::repeat(self.profiles_regul.len(), 1000.0_f32);
        // First guess with all weights at 1.
        let first_guess = DVector::repeat(self.profiles_regul.len(), 1.0_f32);
        // Objective function definition
        let f = |v: DVector<f32>| self.objective_function(v);
        dbg!(&first_guess);

        // Solver call
        let solution = ParticleSwarm::new(f, lower_bounds, upper_bounds).solve(first_guess);

        // Propagate the error or convert to vector
        match solution {
            Err(e) => Err(e),
            Ok(matrix) => Ok(matrix.data.as_vec().to_vec()),
        }
    }
}
