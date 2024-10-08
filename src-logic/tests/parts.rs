use ndarray::SliceInfoElem;
use plotters::style::BLACK;
use plotters::style::BLUE;
use plotters::style::GREEN;
use src_logic::slide::SlideConfig;
use src_logic::slide::SlideMethod;
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
        assert_approx_eq!(dem.surface.z[k], expect_z[k]);
    }
}

#[test]
fn test_plot_dem() {
    use src_logic::plotter::*;

    let dem_path = String::from("./test_data/dem.csv");
    let slbl_path = String::from("./test_data/slbl.csv");
    let dem = Dem1D::from_csv(dem_path);
    let surface = Surface1D::from_csv(slbl_path);

    let graph_buffer = plot_section((1024, 768), (&dem, get_style(BLACK, 1., true, 2)), vec![(&surface, get_style(BLUE, 1., true, 1))], vec![], vec![]);
    graph_buffer.save("figures/test1.png").unwrap();
}

#[test]
fn test_plot_arrows() {
    use src_logic::plotter::*;

    let dem_path = String::from("./test_data/dem.csv");
    let slbl_path = String::from("./test_data/slbl.csv");
    let dem = Dem1D::from_csv(dem_path);
    let mut surface = Surface1D::from_csv(slbl_path);

    let slide_config = SlideConfig::new(SlideMethod::ExactMatrix, 1, 4, 2.0);
    let surface2 = Surface1D::from_slbl(&slide_config, &dem);

    let mut disp = DispProfile::from_surface(&mut surface, &dem, 1, 5);
    let mut disp2 = DispProfile::from_surface(&mut surface, &dem, 1, 4);
    disp.amplitude_vec = disp.amplitude_vec.iter().map(|k| k * 10.).collect();
    disp2.amplitude_vec = disp2.amplitude_vec.iter().map(|k| k * 20.).collect();

    let graph_buffer = plot_section((1024, 768), (&dem, get_style(BLACK, 1., true, 2)), vec![(&surface, get_style(BLUE, 1., true, 1)), (&surface2, get_style(GREEN, 1., true, 1))], vec![&disp, &disp2], vec![]);
    graph_buffer.save("figures/test2.png").unwrap();
}
