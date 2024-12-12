use crate::{data::vec_proj::Vector2Rep, data::ComposedModel, types::*};
pub mod disp;
use disp::*;

impl DispProfile {
    /// Apply a single weight coefficient to all vectors amplitude
    pub fn weight_disp(&mut self, weight: f32) -> &Self {
        for v in 0..self.vecs.len() {
            self.vecs[v].multiply(weight);
        }
        self
    }

    /// Interpolate the vectors on a new set of origin points
    pub fn interpolate_on_origins(&mut self, origins: &Vec<[f32; 2]>) -> &Self {
        log::trace!("interpolate on origins");
        let new_origins = origins.to_owned();
        let mut x_old = vec![];
        let mut vx_old = vec![];
        let mut vz_old = vec![];
        let mut x_new = vec![];
        for k in 0..self.vecs.len() {
            let coords = self.vecs[k].coords();
            vx_old.push(coords.0);
            vz_old.push(coords.1);
            x_old.push(self.origins[k][0]);
        }
        for k in 0..new_origins.len() {
            x_new.push(new_origins[k][0]);
        }
        let new_vx = interpol_linear(&x_old, &vx_old, &x_new);
        let new_vz = interpol_linear(&x_old, &vz_old, &x_new);
        self.origins = new_origins;
        let mut new_vecs = Vec::with_capacity(self.origins.len());
        for k in 0..self.origins.len() {
            new_vecs.push(Vector2Rep::new(new_vx[k], new_vz[k]));
        }
        self.vecs = new_vecs;
        self
    }

    /// Interpolate n vectors from the existing vectors to condition the display
    pub fn interpolate_n_vec(&mut self, n_vec: usize) -> &Self {
        log::trace!("interpolate n vec");
        let x_step =
            (self.origins.last().unwrap()[0] - self.origins.first().unwrap()[0]) / (n_vec as f32);
        let old_x = self.origins.iter().map(|[x, _y]| *x).collect();
        let old_z = self.origins.iter().map(|[_x, y]| *y).collect();
        let new_x: Vec<f32> = (0..n_vec)
            .map(|k| self.origins.last().unwrap()[0] + x_step * (k as f32))
            .collect();
        let new_z = interpol_linear(&old_x, &old_z, &new_x);
        let new_origins: Vec<[f32; 2]> = (0..new_x.len()).map(|k| [new_x[k], new_z[k]]).collect();
        self.interpolate_on_origins(&new_origins)
    }

    /// Construct a disp profile directly from a surface
    pub fn from_surface(
        surface: &mut Surface1D,
        dem: &Dem1D,
        first_x: usize,
        last_x: usize,
    ) -> Result<Self, VectorInputError> {
        log::trace!("Construct a disp profile directly from a surface, slope can be undefined");
        if surface.slope.is_none() {
            surface.get_slope(dem);
            log::debug!("slope defined");
        }
        Self::from_surface_with_slope(surface, dem, first_x, last_x)
    }

    /// Construct a disp profile directly from a surface
    pub fn from_surface_with_slope(
        surface: &Surface1D,
        dem: &Dem1D,
        first_x: usize,
        last_x: usize,
    ) -> Result<Self, VectorInputError> {
        log::trace!("Try constructing a disp profile from surface");
        let slope = surface.slope.clone().unwrap();
        let len = slope.len();
        let origin = match pillar_slope(first_x, last_x, &surface.z, &slope, &dem.x, &dem.surface.z)
        {
            Err(_) => {
                log::error!("Failed to construct disp profile from surface");
                return Err(VectorInputError::PillarError);
            }
            Ok(o) => o,
        };
        let mut amplitude: Vec<f32> = Vec::new();
        (0..len).for_each(|k| match k {
            k if k < first_x => amplitude.push(0.),
            k if k > last_x => amplitude.push(0.),
            _ => amplitude.push(1.),
        });

        let is_right = surface.z[last_x] < surface.z[first_x];

        DispProfile::from_slope_params(slope, amplitude, origin.0, origin.1, is_right)
    }

    pub fn from_surface_direct(
        surface: &mut Surface1D,
        dem: &Dem1D,
    ) -> Result<Self, VectorInputError> {
        log::trace!("disp from surface direct / all points");
        Self::from_surface(surface, dem, 1, dem.x.len() - 2)
    }

    /// Apply a defined gradient onto the vectors amplitude
    pub fn apply_amplitude_gradient(&mut self, gradient: &Vec<(usize, f32)>) {
        let current_amplitudes: Vec<f32> = self.vecs.iter().map(|vec| vec.amplitude()).collect();
        let gradient_amp = amplitude_gradient(&current_amplitudes, gradient);
        for k in 0..self.vecs.len() {
            self.vecs[k].with_norm(gradient_amp[k]);
        }
    }

    /// Create a new profile by combining multiples surfaces responses with known weights
    pub fn from_surfaces(
        dem: &Dem1D,
        surfaces: &mut Vec<Surface1D>,
        boundaries: &Vec<[usize; 2]>,
        gradient: &Vec<Vec<(usize, f32)>>,
        weights: &Vec<f32>,
    ) -> Result<Self, VectorInputError> {
        log::trace!("profile from surfaces combination");
        let regul_origins: Vec<[f32; 2]> = (0..dem.x.len())
            .map(|k| [dem.x[k], dem.surface.z[k]])
            .collect();

        let mut sum_vx = vec![0.; regul_origins.len()];
        let mut sum_vz = vec![0.; regul_origins.len()];

        for surf in 0..surfaces.len() {
            // Create the profile from surface
            let mut current_unit_profile = DispProfile::from_surface(
                &mut surfaces[surf],
                dem,
                boundaries[surf][0],
                boundaries[surf][1],
            )?;
            // Apply the gradient to the unit profile
            if !gradient[surf].is_empty() {
                current_unit_profile.apply_amplitude_gradient(&gradient[surf]);
            }
            // Apply the weight to the unit profile
            current_unit_profile.weight_disp(weights[surf]);
            // Interpolate vectors on common regulate origins
            current_unit_profile.interpolate_on_origins(&regul_origins);

            for k in 0..sum_vx.len() {
                sum_vx[k] += current_unit_profile.vecs[k].coords().0;
                sum_vz[k] += current_unit_profile.vecs[k].coords().1;
            }
        }

        let vecs = (0..regul_origins.len())
            .map(|k| Vector2Rep::new(sum_vx[k], sum_vz[k]))
            .collect();

        DispProfile::new(vecs, regul_origins)
    }

    pub fn from_solver(
        dem: &Dem1D,
        surfaces: &Vec<Surface1D>,
        boundaries: &Vec<[usize; 2]>,
        gradient: &Vec<Vec<(usize, f32)>>,
        disp_data: &DispData,
        section_orientation: &Orientation,
        los_orientation: &Orientation,
    ) -> Result<(Self, Vec<f32>), VectorInputError> {
        log::trace!("profile using particule swarm solver for weight optimiz");
        let regul_origins: Vec<[f32; 2]> = (0..dem.x.len())
            .map(|k| [dem.x[k], dem.surface.z[k]])
            .collect();

        let mut sum_vx = vec![0.; regul_origins.len()];
        let mut sum_vz = vec![0.; regul_origins.len()];

        let mut updated_profiles = vec![];

        for surf in 0..surfaces.len() {
            // Create the profile from surface
            let mut current_unit_profile = DispProfile::from_surface_with_slope(
                &surfaces[surf],
                dem,
                boundaries[surf][0],
                boundaries[surf][1],
            )?;
            // Apply the gradient to the unit profile
            if !gradient[surf].is_empty() {
                current_unit_profile.apply_amplitude_gradient(&gradient[surf]);
            }
            // Interpolate vectors on common regulate origins
            current_unit_profile.interpolate_on_origins(&regul_origins);
            updated_profiles.push(current_unit_profile);
        }

        let composed_model = ComposedModel::new(
            dem,
            &updated_profiles,
            section_orientation,
            los_orientation,
            disp_data,
        );
        match composed_model.fit_disp() {
            Ok(weights) => {
                #[cfg(test)]
                dbg!(&weights);

                for p in 0..updated_profiles.len() {
                    updated_profiles[p].weight_disp(weights[p]);
                    for k in 0..sum_vx.len() {
                        sum_vx[k] += updated_profiles[p].vecs[k].coords().0;
                        sum_vz[k] += updated_profiles[p].vecs[k].coords().1;
                    }
                }

                let vecs = (0..regul_origins.len())
                    .map(|k| Vector2Rep::new(sum_vx[k], sum_vz[k]))
                    .collect();

                match DispProfile::new(vecs, regul_origins) {
                    Err(e) => Err(e),
                    Ok(p) => Ok((p, weights)),
                }
            }
            Err(e) => Err(VectorInputError::SolverError(e)),
        }
    }
}

/// Direct linear interpolation between an old sampling to a new one
pub(crate) fn interpol_linear(x_old: &Vec<f32>, y_old: &Vec<f32>, x_new: &Vec<f32>) -> Vec<f32> {
    log::trace!("linear interpolation");
    let length = x_old.len();
    assert_eq!(x_old.len(), y_old.len());

    let mut left_index: usize = 1;
    let mut y_new: Vec<f32> = Vec::new();

    (0..x_new.len()).for_each(|k| {
        let y = match x_new[k] {
            x if x <= x_old[0] => y_old[0],
            x if x >= x_old[length - 1] => y_old[length - 1],
            x if x < x_old[left_index] => interpol_linear_local(
                x_old[left_index - 1],
                x_old[left_index],
                y_old[left_index - 1],
                y_old[left_index],
                x_new[k],
            ),
            x if x > x_old[left_index] => {
                while (x >= x_old[left_index]) & (left_index < length - 1) {
                    left_index += 1;
                }
                interpol_linear_local(
                    x_old[left_index - 1],
                    x_old[left_index],
                    y_old[left_index - 1],
                    y_old[left_index],
                    x_new[k],
                )
            }
            _ => y_old[left_index],
        };
        y_new.push(y);
    });
    y_new
}

/// Direct local linear interpolation between two points
fn interpol_linear_local(x1: f32, x2: f32, y1: f32, y2: f32, xn: f32) -> f32 {
    y1 + (xn - x1) * (y2 - y1) / (x2 - x1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_local_interp() {
        let result = interpol_linear_local(1., 5., 10., 35., 4.);
        let expect: f32 = 28.75;
        assert_approx_eq!(result, expect);
    }

    #[test]
    fn test_linear_interp() {
        let x: Vec<f32> = vec![1., 3., 5., 7., 9., 11., 13.];
        let y: Vec<f32> = vec![12., 56., 23., 45., 56., 45., 23.];
        let x_new: Vec<f32> = vec![0., 2., 4., 6., 8., 10.];
        let result = interpol_linear(&x, &y, &x_new);
        let expect: Vec<f32> = vec![12., 34., 39.5, 34., 50.5, 50.5];
        println!("result: {:?}\nexpect: {:?}", result, expect);
        (0..result.len()).for_each(|k| assert_approx_eq!(result[k], expect[k]));

        let x: Vec<f32> = vec![1., 3., 5., 7., 9., 11., 13.];
        let y: Vec<f32> = vec![12., 56., 23., 45., 56., 45., 23.];
        let x_new: Vec<f32> = vec![4., 6., 8., 10., 12., 14., 16.];
        let result = interpol_linear(&x, &y, &x_new);
        let expect: Vec<f32> = vec![39.5, 34., 50.5, 50.5, 34., 23., 23.];
        println!("result: {:?}\nexpect: {:?}", result, expect);
        (0..result.len()).for_each(|k| assert_approx_eq!(result[k], expect[k]));
    }
}

#[cfg(test)]
mod test_fitter {
    use super::*;

    #[test]
    fn test_fitter() {
        let x: Vec<f32> = (0..=10).map(|k| 2. * k as f32).collect();
        let section_orientation = Orientation::from_deg(260., 90.).unwrap();
        let los_orientation = Orientation::from_deg(286., 35.).unwrap();
        let surf_topo = vec![1., 2., 2., 4., 5., 5., 6., 7., 9., 10., 10.];
        let dem = Dem1D {
            x,
            surface: Surface1D::new(surf_topo),
        };
        let surf1 = vec![1., 2., 2., 3., 4., 5., 6., 7., 9., 10., 10.];
        let surf2 = vec![1., 2., 2., 2., 3., 3., 4., 5., 6., 10., 10.];
        let mut surf1 = Surface1D::new(surf1);
        let mut surf2 = Surface1D::new(surf2);

        let disp_data = DispData::new(vec![6., 10., 12., 17.], vec![-5., -3., -2., -0.5]).unwrap();
        let profile1 = DispProfile::from_surface(&mut surf1, &dem, 2, 5);
        assert!(profile1.is_ok());
        let profile2 = DispProfile::from_surface(&mut surf2, &dem, 2, 9);
        assert!(profile2.is_ok());

        let _profile = DispProfile::from_solver(
            &dem,
            &mut vec![surf1, surf2],
            &vec![[2, 5], [2, 9]],
            &vec![vec![], vec![]],
            &disp_data,
            &section_orientation,
            &los_orientation,
        )
        .unwrap();
        // for k in 0..profile.vecs.len() {
        //     dbg!((profile.vecs[k]));
        //     dbg!(profile.vecs[k].amplitude());
        // }
    }
}
