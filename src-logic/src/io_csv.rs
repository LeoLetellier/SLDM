use crate::prelude::*;

fn read_csv(file: String) -> Vec<Vec<f64>> {
    Default::default()
}

pub trait FromCSV {
    fn from_csv(file: String) -> Self;
}

impl FromCSV for Dem1D {
    fn from_csv(file: String) -> Self {
        let values = read_csv(file);
        if values.len() > 2 {
            panic!("Too much columns in CSV. Expected 2.")
        }
        let mut dem = Dem1D::default();
        dem.x = values[0].clone();
        dem.z = values[1].clone();
        dem
    }
}

impl FromCSV for Surface1D {
    fn from_csv(file: String) -> Self {
        let values = read_csv(file);
        if values.len() > 2 {
            panic!("Too much columns in CSV. Expected 2.")
        }
        let mut surface = Surface1D::default();
        surface.z = values[1].clone();
        surface
    }
}
