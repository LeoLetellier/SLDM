//! Handles project abstraction over operation on section objects
//! 
//! Implement abstraction over properties on the same section and project (with Project Unit), and 
//! capacity to handle multiple projects at once.


use crate::{model::DispModel, slide::{Slide, SlideConfig, SlideMethod}, prelude::{self, Dem1D, Localisation, PropOnSection}};
pub(crate) mod infile;
use std::sync::Arc;


/// Project session is a manager of Project Unit. It enables handling multiples 
/// project under a single execution of the program.
/// This means you can load multiple sections at once.
pub struct ProjectSession {
    projects: Vec<ProjectUnit>,
}

impl Default for ProjectSession {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectSession {
    pub fn new() -> ProjectSession {
        ProjectSession {
            projects: Vec::<ProjectUnit>::new(),
        }
    }

    pub fn new_project(&mut self, name: String, dem_path: String) {
        if self.get_project_index(&name).is_some() {
            panic!("Cannot add new project, name already taken")
        }
        self.projects.push(ProjectUnit::new_from_dem(name, dem_path))
    }

    pub fn get_project_index(&self, name_to_search: &String) -> Option<usize> {
        for k in 0..self.projects.len() {
            if self.projects[k].name == *name_to_search {
                return Some(k)
            }
        }
        None
    }

    pub fn delete_project(&mut self, name_to_delete: &String) {
        let id = self.get_project_index(name_to_delete);
        match id {
            None => panic!("Cannot delete, no corresponding project"),
            Some(i) => self.projects.swap_remove(i),
        };
    }

    pub fn access_project(&self, name_to_access: &String) -> &ProjectUnit {
        let id = self.get_project_index(name_to_access);
        match id {
            Some(i) => &self.projects[i],
            _ => panic!("Cannot access, no corresponding project"),
        }
    }

    pub fn access_mut_project(&mut self, name_to_access: &String) -> &mut ProjectUnit {
        let id = self.get_project_index(name_to_access);
        match id {
            Some(i) => &mut self.projects[i],
            _ => panic!("Cannot access, no corresponding project"),
        }
    }
}

pub struct ProjectUnit {
    // manage adding, removing, moving parts in projects, and projects themselves
    pub(crate) name: String,
    pub(crate) dem: Arc<Dem1D>,
    pub slides: Vec<Arc<Slide>>,
    pub model: Option<DispModel>,
    pub sar_disp: Option<Vec<PropOnSection>>,
    pub orientation: Option<prelude::Orientation>,
    pub localisation: Option<prelude::Localisation>,
}

impl ProjectUnit {
    fn new(name:String, dem: Dem1D) -> ProjectUnit {
        ProjectUnit {
            name,
            dem: Arc::new(dem),
            slides: vec![],
            model: None, // consider handling multiple models
            sar_disp: None,
            orientation: None,
            localisation: None,
        }
    }

    fn new_from_dem(name: String, dem_path: String) -> ProjectUnit {
        let dem = prelude::Dem1D::read_from_file(dem_path);
        Self::new(name, dem)
    }

    pub fn new_slide(&mut self, method: SlideMethod, first_pnt: usize, last_pnt: usize, tolerance: f64) {
        let config = SlideConfig::new(method, first_pnt, last_pnt, tolerance);
        let mut slide: Slide = Slide::new(Arc::clone(&self.dem), config);
        slide.update_slide();
        slide.update_slope_and_vec();
        slide.update_pilar();
        self.slides.push(Arc::new(slide));
    }

    pub fn compute_displacement_model(&mut self, slide_list: &Vec<Arc<Slide>>, main_slide: Arc<Slide>) {
        let len = self.dem.len;
        self.model = Some(DispModel::build(slide_list, main_slide, len));
    }

    pub fn compute_displacement_model_default(&mut self) {
        self.model = Some(DispModel::build(&self.slides, Arc::clone(&self.slides.first().unwrap()), self.dem.len));
    }

    pub fn set_orientation(&mut self, azimuth: f64) {
        self.orientation = Some(prelude::Orientation::new(azimuth, 0.));
    }

    pub fn set_location(&mut self, lat1: f64, long1: f64, lat2: f64, long2: f64) {
        self.localisation = Some(Localisation::new(lat1, long1, lat2, long2));
    }
}

#[cfg(test)]
mod tests {
    use super::{ProjectSession, ProjectUnit};

    #[test]
    fn project_dem() {
        let result = ProjectUnit::new_from_dem(String::from("Test"), String::from("test_data/dem.csv"));
        let expect_length = 7;
        assert_eq!(result.dem.len, expect_length);
    }

    #[test]
    fn create_proj() {
        let mut session = ProjectSession::new();
        let path = String::from("test_data/dem.csv");
        session.new_project(String::from("proj1"), path.to_owned());
        session.new_project(String::from("proj2"), path.to_owned());
        let res = session.get_project_index(&String::from("proj2"));
        let expect = Some(1);
        assert_eq!(res, expect);

        session.delete_project(&String::from("proj1"));
        let res = session.get_project_index(&String::from("proj2"));
        let expect = Some(0);
        assert_eq!(res, expect);
    }

}