use std::f32::consts::PI;

pub fn test_lib() -> String {
    String::from("Hello Lib!")
}

#[derive(Default, Debug, Clone)]
pub struct Dem1D {
    pub orientation: Orientation,
    pub x: Vec<f32>,
    pub surface: Surface1D,
}

impl Dem1D {
    pub(crate) fn new(orientation: Orientation, x: Vec<f32>, z: Vec<f32>) -> Self {
        if x.len() != z.len() {
            panic!("Length of x and z vectors does not match when creating Dem struct.")
        } else {
            let surface = Surface1D::new(z);
            Dem1D {orientation, x, surface}
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Surface1D {
    pub z: Vec<f32>,
    pub slope: Option<Vec<f32>>,
}

impl Surface1D {
    pub fn new(z: Vec<f32>) -> Self {
        Surface1D { z: z, slope: None }
    }
}

#[derive(Default, Debug)]
pub struct DispProfile {
    pub slope_vec: Vec<f32>,
    pub amplitude_vec: Vec<f32>,
    pub origin_x: Vec<f32>,
    pub origin_z: Vec<f32>,
    // portion seen into a LOS
    pub proj_slope_vec: Option<Vec<f32>>,
    pub proj_amplitude_vec: Option<Vec<f32>>,
}

impl DispProfile {
    pub fn new(slope_vec: Vec<f32>, amplitude_vec: Vec<f32>, origin_x: Vec<f32>, origin_z: Vec<f32>) -> Self {
        DispProfile {
            slope_vec: slope_vec,
            amplitude_vec: amplitude_vec,
            origin_x: origin_x,
            origin_z: origin_z,
            proj_slope_vec: None,
            proj_amplitude_vec: None,
        }
    }

    pub fn get_xz_vec(&self) -> (Vec<f32>, Vec<f32>) {
        // the slope gives the line, but not the orientation on the line
        // q : vertical ?
        // let cor = match facing_right {
        //     true => 1,
        //     _ => -1,
        // };
        let vec_x: Vec<f32> = (0..self.slope_vec.len()).map(|k| self.slope_vec[k].cos() * self.amplitude_vec[k] * self.slope_vec[k].signum()).collect();
        let vec_z: Vec<f32> = (0..self.slope_vec.len()).map(|k| self.slope_vec[k].sin() * self.amplitude_vec[k] * self.slope_vec[k].signum()).collect();
        (vec_x, vec_z)
    }
}

#[derive(Default, Debug)]
pub struct DispData {
    pub x: Vec<f32>,
    pub orientation: Orientation,
    pub amplitude: Vec<f32>,
    // portion seen into the 2D section
    pub proj_slope_vec: Option<Vec<f32>>,
    pub proj_amplitude_vec: Option<Vec<f32>>,
}

impl DispData {
    // pub fn get_xz_vec(&self) -> (Vec<f32>, Vec<f32>) {
    //     let vec_x: Vec<f32> = (0..self.slope_vec.len()).map(|k| self.slope_vec[k].cos() * self.amplitude_vec[k]).collect();
    //     let vec_z: Vec<f32> = (0..self.slope_vec.len()).map(|k| self.slope_vec[k].sin() * self.amplitude_vec[k]).collect();
    //     (vec_x, vec_z)
    // }
}

#[derive(Default, Debug)]
struct DispModel {
    pub profile: DispProfile,
    pub data: DispData,
}


#[derive(Debug, Default, PartialEq, Clone)]
pub struct Orientation {
    pub(crate) azimuth : f32,
    pub(crate) incidence : f32,
}

impl Orientation {
    pub fn new(azimuth: f32, incidence: f32) -> Self {
        if !check_azimuth_range(azimuth) {
            panic!("Range of azimuth is incorrect when creating Orientation struct.")
        } else if !check_incidence_range(incidence) {
            panic!("Range of incidence is incorrect when creating Orientation struct.")
        } else {
            Orientation { azimuth, incidence }
        }
    }
}

fn check_azimuth_range(azimuth: f32) -> bool {
    match azimuth {
        s if s < 0. => false,
        s if s > 2. * PI => false,
        _ => true,
    }
}

fn check_incidence_range(incidence: f32) -> bool {
    match incidence {
        d if d < 0. => false,
        d if d > PI => false,
        _ => true,
    }
}
