use std::f32::consts::PI;

pub fn test_lib() -> String {
    String::from("Hello Lib!")
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Dem2D {
    pub orientation: Orientation,
    pub x: Vec<f32>,
    pub z: Vec<f32>,
    pub len: usize,
}

impl Dem2D {
    pub(crate) fn new(orientation: Orientation, x: Vec<f32>, z: Vec<f32>) -> Self {
        if x.len() != z.len() {
            panic!("Length of x and z vectors does not match when creating Dem struct.")
        } else {
            let len = x.len();
            Dem2D {orientation, x, z, len}
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Surface2D {
    pub z: Vec<f32>,
    pub slope: Option<Vec<f32>>,
}

#[derive(Default, Debug)]
pub struct DispProfile {
    pub slope: Vec<f32>,
    pub amplitude: Vec<f32>,
    pub origin: Vec<Point2D>,
    // portion seen into a LOS
    pub proj_slope: Option<Vec<f32>>,
    pub proj_amplitude: Option<Vec<f32>>,
}

#[derive(Default, Debug)]
pub struct DispData {
    pub x: Vec<f32>,
    pub orientation: Orientation,
    pub amplitude: Vec<f32>,
}

#[derive(Default, Debug)]
pub struct Point2D {
    x: f32,
    y: f32,
}

impl Point2D {
    pub fn new(x: f32, y: f32) -> Self {
        Point2D {
            x: x,
            y: y,
        }
    }
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
