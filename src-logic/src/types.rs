//! Implement basic struct for the crate to handle properties on section

use crate::project::infile::FileReader;
use std::f64::consts::PI;

/// Check function for testing extern use
pub fn test_lib() -> String {
    String::from("Hello Lib!")
}

/// Centralize DEM (Digital Elevation Model) data
#[derive(PartialEq, Debug, Clone)]
pub struct Dem {
    pub x: Vec<f64>,
    pub z: Vec<f64>,
    pub len: usize,
}

impl Dem {
    /// Initialize the dem struct with x and z inputs
    /// 
    /// * x: position or map location on the section (x-axis)
    /// * z: elevation corresponding to each map location (z-axis)
    pub(crate) fn new(x: Vec<f64>, z: Vec<f64>) -> Dem {
        if x.len() != z.len() {
            panic!("Length of x and z vectors does not match when creating Dem struct.")
        } else {
            let len = x.len();
            Dem { x, z, len, }
        }
    }

    /// Initialize the dem struct directly from an input file
    pub fn read_from_file(path: String) -> Dem {
        let reader = FileReader::new(path, ';', 1);
        let res = reader.parse_unpack();
        assert_eq!(res.len(), 2);
        
        Self::new(res[0].to_owned(), res[1].to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::Dem;

    #[test]
    fn create_dem() {
        let result = Dem::read_from_file(String::from("test_data/dem.csv"));
        let vec_expect_x = vec![0., 100., 200., 300., 400., 500., 600.];
        let vec_expect_z = vec![0., 10., 30., 35., 45., 50., 60.];
        let expect = Dem::new(vec_expect_x, vec_expect_z);
        assert_eq!(result, expect);
    }
}


/// Define 3D geometry for a reference line
/// 
/// In this 3D geometry, azimuth refers to the map orientation of the line (between 0 and 2pi in radians), 
/// incidence refers to the angle defined between the line and the vertical axis (facing down).
/// 
/// For a section: 
/// * azimuth: direction pointed by the x-axis
/// * incidence: 90° or pi/2 (as the reference line is defined along the x-axis), or the slope when 
/// considering vectors into the section. The angle is defined right-wise, for positive x increment.
/// 
/// For SAR geometry:
/// * azimuth: LOS azimuth (satellite heading + 90°, ascending or descending)
/// * incidence: LOS incidence
#[derive(Debug)]
pub struct Orientation {
    pub(crate) azimuth : f64,
    pub(crate) incidence : f64,
}

impl Orientation {
    /// Initialize orientation struct with azimuth and incidence parameters
    pub fn new(azimuth: f64, incidence: f64) -> Orientation {
        if !check_azimuth_range(azimuth) {
            panic!("Range of azimuth is incorrect when creating Orientation struct.")
        } else if !check_incidence_range(incidence) {
            panic!("Range of incidence is incorrect when creating Orientation struct.")
        } else {
            Orientation { azimuth, incidence }
        }
    }
}

/// Check if the given input range is acceptable for an azimuth angle
fn check_azimuth_range(azimuth: f64) -> bool {
    match azimuth {
        s if s < 0. => false,
        s if s > 2. * PI => false,
        _ => true,
    }
}

/// Check if the given input range is acceptable for an incidence angle
fn check_incidence_range(incidence: f64) -> bool {
    match incidence {
        d if d < 0. => false,
        d if d > PI => false,
        _ => true,
    }
}

/// Wraper around a vector of floats for handling properties on a section
#[derive(Clone, Debug)]
// Consider removing PropOnSection struct and replace by Vec<f64>
// or refractoring to conserve impl
pub struct PropOnSection {
    name: String,
    pub prop: Vec<f64>,
    // TODO uid
}

impl PropOnSection {
    /// Initialize the struct PropOnSection given its name and a vector of floats
    pub fn new(name: String, prop: Vec<f64>) -> PropOnSection {
        PropOnSection {
            name,
            prop,
        }
    }

    /// Interpolate the properties on a new set of points. Can be used to match DEM discretization.
    /// 
    /// will change
    pub fn interpolate_prop(&self, dem: Dem, x: f64) -> Option<f64> {
        // return None if outside of range of dem
        let mut interp_range: usize = 0;
        let mut z_interp: Option<f64> = None;
        while interp_range < dem.len - 2 {
            if x > dem.x[interp_range] && x < dem.x[interp_range + 1] {
                z_interp = Some(dem.z[interp_range] + ((dem.z[interp_range + 1] - dem.z[interp_range]) /
                 (dem.x[interp_range + 1] - dem.x[interp_range]) * (dem.x[interp_range + 1] - x)));
                break;
            }
            interp_range += 1;
        };
        z_interp
    }

}

/// Regroup GPS coordinates for the starting and ending points of a section
#[derive(Debug)]
pub struct Localisation{
    lat_1: f64,
    long_1: f64,
    lat_2: f64,
    long_2: f64,
}

impl Localisation {
    /// Initialize the struct localization
    pub(crate) fn new(lat1: f64, long1: f64, lat2: f64, long2: f64) -> Localisation {
        Localisation {
            lat_1: lat1,
            long_1: long1,
            lat_2: lat2,
            long_2: long2,
        }
    }
}
