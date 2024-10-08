use src_logic::types::*;
use src_logic::io_csv::*;
use assert_approx_eq::assert_approx_eq;

#[test]
fn test_dem_input() {
    let file_path = String::from("./test_data/dem.csv");
    let dem = Dem1D::from_csv(file_path);
    let expect_x: Vec<f32> = vec![0., 100., 200., 300., 400., 500., 600.];
    let expect_z: Vec<f32> = vec![0., 10., 30., 35., 45., 50., 60.];
    for k in 0..dem.x.len() {
        assert_approx_eq!(dem.x[k], expect_x[k]);
        assert_approx_eq!(dem.z[k], expect_z[k]);
    }
}

#[test]
fn test_plot_dem() {
    use src_logic::plotter::*;

    let dem_path = String::from("./test_data/dem.csv");
    let slbl_path = String::from("./test_data/slbl.csv");
    let dem = Dem1D::from_csv(dem_path);
    let surface = Surface1D::from_csv(slbl_path);

    let graph_buffer = plot_section((1024, 768), &dem, vec![&surface], vec![], vec![]);
    graph_buffer.save("figures/test1.png").unwrap();
}

#[test]
fn test_plot_arrows() {
    use src_logic::plotter::*;

    let dem_path = String::from("./test_data/dem.csv");
    let slbl_path = String::from("./test_data/slbl.csv");
    let dem = Dem1D::from_csv(dem_path);
    let surface = Surface1D::from_csv(slbl_path);

    let graph_buffer = plot_section((1024, 768), &dem, vec![&surface], vec![], vec![]);
    graph_buffer.save("figures/test2.png").unwrap();
}