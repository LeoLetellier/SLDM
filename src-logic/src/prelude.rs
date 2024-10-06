use std::f32::consts::PI;

pub fn test_lib() -> String {
    String::from("Hello Lib!")
}

#[derive(Default, PartialEq, Debug, Clone)]
pub struct Dem1D {
    pub name: String,
    pub orientation: Orientation,
    pub x: Vec<f32>,
    pub z: Vec<f32>,
    pub len: usize,
}

impl Dem1D {
    pub(crate) fn new(name: String, orientation: Orientation, x: Vec<f32>, z: Vec<f32>) -> Self {
        if x.len() != z.len() {
            panic!("Length of x and z vectors does not match when creating Dem struct.")
        } else {
            let len = x.len();
            Dem1D {name, orientation, x, z, len}
        }
    }

    // pub fn read_from_file(path: String) -> Self {
    //     let reader = FileReader::new(path, ';', 1);
    //     let res = reader.parse_unpack();
    //     assert_eq!(res.len(), 2);
        
    //     Self::new(String::from("DEM"), Orientation::default(), res[0].to_owned(), res[1].to_owned())
    // }
}

#[derive(Default, Debug, Clone)]
pub struct Surface1D {
    pub name: String,
    pub z: Vec<f32>,
    pub slope: Option<Vec<f32>>,
}

pub struct DispProfile {
    pub name: String,
    pub pilar_grounded: (Vec<f32>, Vec<f32>),

}

pub struct DispData {
    pub name: String,
}

pub struct Vec1D {
    // a vector is defined from it's orientaiton (slope) and amplitude (arrow length)
    // can be converted to x,z components
    pub slope: Vec<f32>,
    pub amplitude: Vec<f32>,
}

// #[cfg(test)]
// mod tests {
//     use super::Dem1D;

//     #[test]
//     fn create_dem() {
//         let result = Dem1D::read_from_file(String::from("test_data/dem.csv"));
//         let vec_expect_x = vec![0., 100., 200., 300., 400., 500., 600.];
//         let vec_expect_z = vec![0., 10., 30., 35., 45., 50., 60.];
//         let expect = Dem1D::new(vec_expect_x, vec_expect_z);
//         assert_eq!(result, expect);
//     }
// }


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

// #[derive(Clone, Debug)]
// pub struct PropOnSection {
//     name: String,
//     pub prop: Vec<f32>
// }

// impl PropOnSection {
//     pub fn new(name: String, prop: Vec<f32>) -> PropOnSection {
//         PropOnSection {
//             name,
//             prop,
//         }
//     }

//     pub fn interpolate_prop(&self, dem: Dem, x: f32) -> Option<f32> {
//         let mut interp_range: usize = 0;
//         let mut z_interp: Option<f32> = None;
//         while interp_range < dem.len - 2 {
//             if x > dem.x[interp_range] && x < dem.x[interp_range + 1] {
//                 z_interp = Some(dem.z[interp_range] + ((dem.z[interp_range + 1] - dem.z[interp_range]) /
//                  (dem.x[interp_range + 1] - dem.x[interp_range]) * (dem.x[interp_range + 1] - x)));
//                 break;
//             }
//             interp_range += 1;
//         };
//         z_interp
//     }

// }

// #[derive(Debug)]
// pub struct Localisation{
//     lat_1: f32,
//     long_1: f32,
//     lat_2: f32,
//     long_2: f32,
// }

// impl Localisation {
//     pub(crate) fn new(lat1: f32, long1: f32, lat2: f32, long2: f32) -> Localisation {
//         Localisation {
//             lat_1: lat1,
//             long_1: long1,
//             lat_2: lat2,
//             long_2: long2,
//         }
//     }
// }
