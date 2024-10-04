use src_logic::project::*;
use src_logic::slide::*;
use src_logic::types::Orientation;

#[test]
fn workflow() {
    let mut session = ProjectSession::new();
    let path = String::from("test_data/dem.csv");
    let project_name = String::from("example_project");
    session.new_project(project_name.to_owned(), path.to_owned());
    session.new_project(String::from("proj2"), path.to_owned());

    let proj: &mut ProjectUnit = session.access_mut_project(&project_name);
    //proj.new_slide(SlideMethod::ExactMatrix, 1, 5, 2.0);
    proj.new_slide(SlideMethod::ExactMatrix, 2, 4, 2.0);
    proj.compute_displacement_model_default();

    proj.set_orientation(1.919);
    let sar = Orientation::new(4.974, 0.610);
    proj.model.as_mut().unwrap().compute_mdl_disp_in_los(proj.orientation.as_ref().unwrap(), &sar);
    
    dbg!(&proj.slides[0]);
    //dbg!(&proj.slides[1]);
    dbg!(&proj.model);
}

#[test]
fn modulo_cos() {
    use std::f64::consts::PI;
    let angle = PI / 7.;
    println!("1: {}", (angle + PI).cos());
    println!("2: {}", (angle - PI).cos());
    println!("1: {}", (angle + PI).sin());
    println!("2: {}", (angle - PI).sin());
}