use crate::types::*;
pub mod disp;
use disp::*;

impl DispProfile {
    pub fn from_surface(surface: &mut Surface1D, dem: &Dem1D, first_x: usize, last_x: usize) -> Self {
        match surface.slope {
            None => {surface.get_slope(dem); ()},
            _ => (),
        };
        let slope = surface.slope.clone().unwrap();
        let len = slope.len();
        let origin = pillar_slope(first_x, last_x, &surface.z, &slope, &dem.x, &dem.surface.z);
        let mut amplitude: Vec<f32> = Vec::new();
        (0..len).for_each(|k| match k {
            k if k < first_x => amplitude.push(0.),
            k if k > last_x => amplitude.push(0.),
            _ => amplitude.push(1.),
        });
        let mut disp_profile = DispProfile::new(slope, amplitude, origin.0, origin.1);
        disp_profile.interp_to_support(dem);
        disp_profile

    }

    fn interp_to_support(&mut self, dem: &Dem1D) -> &Self {
        self.slope_regul = interpol_linear(&self.origin_x, &self.slope_vec, &dem.x);
        self.amplitude_regul = interpol_linear(&self.origin_x, &self.amplitude_vec, &dem.x);
        self
    }

    // TODO
    // pub fn from_profiles(profiles: Vec<DispProfile>, weights: Vec<f32>) -> Self {
    //     assert_eq!(profiles.len(), weights.len());
    //     let length = profiles[0].slope_regul.len();
    //     let mut profile = DispProfile::default();
    //     (0..length).for_each(|k| {
    //         let (vx, vz) = (0..profiles.len()).fold((0., 0.), |acc, p|
    //             slope_ampl_to_vx_vz_unit(profiles[p].slope_regul[k], profiles[p].amplitude_regul[k])
    //         );
    //         let sum_slope = (0..profiles.len()).fold(0., |acc, p|
    //             weights[p] * profiles[p].slope_regul[k]
    //         );
    //         let sum_amplitude = (0..profiles.len()).fold(0., |acc, p|
    //             weights[p] * profiles[p].amplitude_regul[k]
    //         );
    //     });
    // }
}

fn interpol_linear(x_old: &Vec<f32>, y_old: &Vec<f32>, x_new: &Vec<f32>) -> Vec<f32> {
    let length = x_old.len();
    assert_eq!(x_old.len(), y_old.len());

    let mut left_index: usize = 1;
    let mut y_new: Vec<f32> = Vec::new();

    (0..x_new.len()).for_each(|k| {
        let y = match x_new[k] {
            x if x <= x_old[0] => y_old[0],
            x if x >= x_old[length - 1] => y_old[length - 1],
            x if x < x_old[left_index] => {
                interpol_linear_local(x_old[left_index - 1], x_old[left_index], y_old[left_index - 1], y_old[left_index], x_new[k])
            },
            x if x > x_old[left_index] => {
                while !(x < x_old[left_index]) & (left_index < length - 1) {
                    left_index += 1;
                }
                interpol_linear_local(x_old[left_index - 1], x_old[left_index], y_old[left_index - 1], y_old[left_index], x_new[k])
            },
            _ => y_old[left_index],
        };
        y_new.push(y);
    });
    y_new
}

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
        let expect:f32 = 28.75;
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
        (0..result.len()).for_each(|k|
            assert_approx_eq!(result[k], expect[k])
        );

        let x: Vec<f32> = vec![1., 3., 5., 7., 9., 11., 13.];
        let y: Vec<f32> = vec![12., 56., 23., 45., 56., 45., 23.];
        let x_new: Vec<f32> = vec![4., 6., 8., 10., 12., 14., 16.];
        let result = interpol_linear(&x, &y, &x_new);
        let expect: Vec<f32> = vec![39.5, 34., 50.5, 50.5, 34., 23., 23.];
        println!("result: {:?}\nexpect: {:?}", result, expect);
        (0..result.len()).for_each(|k|
            assert_approx_eq!(result[k], expect[k])
        );
    }
}
