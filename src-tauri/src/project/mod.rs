use crate::{ slide::{self, SlideConfig}, types };
pub(crate) mod infile;

// TODO Session
// TODO UID

pub struct ProjectUnit<'a> {
    pub(crate) dem: types::Dem,
    slides: Vec<slide::Slide<'a>>,
    length: f64,
}

impl<'a> ProjectUnit<'_> {
    fn new(dem: types::Dem) -> ProjectUnit<'a> {
        let length = (dem.x[0] - dem.x[dem.x.len() - 1]).abs();
        ProjectUnit {
            dem: dem,
            slides: vec![],
            length: length,
        }
    }

    fn new_from_dem(dem_path: String) -> ProjectUnit<'a> {
        let dem = types::Dem::read_from_file(dem_path);
        Self::new(dem)
    }

    fn new_slide(&self, method: slide::SlideMethod, first_pnt: usize, last_pnt: usize, tolerance: f64) {
        let config = SlideConfig::new(method, first_pnt, last_pnt, tolerance);
        let slide = slide::Slide::new(&self, config);
    }
}

#[cfg(test)]
mod tests {
    use super::ProjectUnit;

    #[test]
    fn project_dem(){
        let result = ProjectUnit::new_from_dem(String::from("test/dem.csv"));
        let expect_length = 600.;
        assert_eq!(result.length, expect_length);
    }

}