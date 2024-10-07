use src_logic::prelude::*;
use src_logic::io_csv::*;
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_dem_input() {
    let file_path = String::from("./test_data/dem.csv");
    let dem = Dem2D::from_csv(file_path);
    let expect_x: Vec<f32> = vec![0., 100., 200., 300., 400., 500., 600.];
    let expect_z: Vec<f32> = vec![0., 10., 30., 35., 45., 50., 60.];
    for k in 0..dem.x.len() {
        assert_approx_eq!(dem.x[k], expect_x[k]);
        assert_approx_eq!(dem.z[k], expect_z[k]);
    }
}