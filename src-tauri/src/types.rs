use crate::project::infile::FileReader;

#[derive(Copy, Clone)]
pub(crate) enum Unit {
    Meter,
    Degree,
    Velocity,
    Unitless,
}

#[derive(PartialEq, Debug)]
pub(crate) struct Dem {
    pub x: Vec<f64>,
    pub z: Vec<f64>,
    pub len: usize,
}

impl Dem {
    fn new(x: Vec<f64>, z: Vec<f64>) -> Dem {
        if x.len() != z.len() {
            panic!("Length of x and z vectors does not match when creating Dem struct.")
        } else {
            let len = x.len();
            Dem { x, z, len, }
        }
    }

    pub fn read_from_file(path: String) -> Dem {
        let reader = FileReader::new(path, ';', 1);
        let res = reader.parse_unpack();
        assert_eq!(res.len(), 2);
        let dem = Self::new(res[0].to_owned(), res[1].to_owned());
        dem
    }
}

#[cfg(test)]
mod tests {
    use super::Dem;

    #[test]
    fn create_dem() {
        let result = Dem::read_from_file(String::from("test/dem.csv"));
        let vec_expect_x = vec![0., 100., 200., 300., 400., 500., 600.];
        let vec_expect_z = vec![0., 10., 30., 35., 45., 50., 60.];
        let expect = Dem::new(vec_expect_x, vec_expect_z);
        assert_eq!(result, expect);
    }
}

pub(crate) struct Orientation {
    strike : f32, // 0 to 360°
    dip : f32, // 0 to 90°
}

impl Orientation {
    fn new(strike: f32, dip: f32) -> Orientation {
        if !check_strike_range(strike) {
            panic!("Range of strike is incorrect when creating Orientation struct.")
        } else if !check_dip_range(dip) {
            panic!("Range of dip is incorrect when creating Orientation struct.")
        } else {
            Orientation { strike, dip, }
        }
    }
}

fn check_strike_range(strike: f32) -> bool {
    match strike {
        s if s < 0. => false,
        s if s > 360. => false,
        _ => true,
    }
}

fn check_dip_range(dip: f32) -> bool {
    match dip {
        d if d < 0. => false,
        d if d > 90. => false,
        _ => true,
    }
}

#[derive(Clone)]
pub(crate) struct PropOnSection {
    name: String,
    pub prop: Vec<f64>, // TODO generic
    // TODO using None values (Option) // not needed
    // TODO replacing Vec<> by table since length is known // not at compilee time though
    unit: Unit, // TODO not needed
    // TODO uid
}

impl PropOnSection {
    pub fn new(name: String, prop: Vec<f64>, unit: Unit) -> PropOnSection {
        PropOnSection {
            name,
            prop,
            unit,
        }
        // consider using "&" to not copy the data
    }

    pub fn interpolate_prop(&self, dem: Dem, x: f64) -> Option<f64> {
        // return None if outside of range of dem
        let interp_range:usize = 0;
        let mut z_interp:Option<f64> = None;
        while interp_range < dem.len - 2 {
            if x > dem.x[interp_range] && x < dem.x[interp_range + 1] {
                z_interp = Some(dem.z[interp_range] + ((dem.z[interp_range + 1] - dem.z[interp_range]) /
                 (dem.x[interp_range + 1] - dem.x[interp_range]) * (dem.x[interp_range + 1] - x)));
                break;
            }
            let interp_range = interp_range + 1;
        };
        z_interp
    }

}

pub(crate) struct Localisation{
    lat_1: usize,
    long_1: usize,
    lat_2: usize,
    long_2: usize,
}

pub(crate) struct Point2D { // TODO
    pub x: f64,
    pub y: f64,
}