use crate::app;

use super::Project;
use serde::{Deserialize, Serialize};
use src_logic::prelude::*;
use anyhow::Result as AResult;
use anyhow::bail;
use thiserror::Error;
use std::fs;
use std::io::Write;
use std::path::Path;
use toml;
use filenamify::filenamify;
use super::*;

impl Project {
    pub(crate) fn save(&self) -> AResult<()> {
        let path = match self.path.clone() {
            Some(p) => p,
            None => bail!(SaveTomlError::NoPath),
        };

        let root_folder = path.to_string() + "/" + filenamify(self.name.to_string()).as_str();
        let path = Path::new(path.as_str());
        
        match Path::new(path).exists() {
            true => fs::create_dir(root_folder.to_string())?,
            false => bail!(SaveTomlError::InvalidPath),
        };

        save_toml(self, &root_folder)?;
        save_all_csv(self, &root_folder)?;
        Ok(())
    }

    fn load(&mut self) -> AResult<()> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum SaveTomlError {
    #[error("No saving path registered")]
    NoPath,
    #[error("The specified saving path is invalid")]
    InvalidPath,
}

pub(crate) fn save_toml(project: &Project, root_folder: &String) -> AResult<()> {
    let project_definition = ProjectFile::from_project(project);
    let toml = toml::to_string(&project_definition)?;

    let mut file = fs::File::create(root_folder.to_string() + "/project.toml")?;
    file.write_all(toml.as_bytes())?;
    
    Ok(())
}

pub(crate) fn save_all_csv(project: &Project, root_folder: &String) -> AResult<()> {
    todo!();
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectFile {
    project: ProjectRelated,
    surface: Option<Vec<SurfaceRelated>>,
    model: Option<Vec<ModelRelated>>,
    disp_data: Option<Vec<DispGeomRelated>>,
}

impl ProjectFile {
    fn from_project(app_project: &Project) -> Self {
        let project = ProjectRelated::from_project(app_project);
        let surface = if app_project.surfaces.is_empty() {
            None
        } else {
            Some(SurfaceRelated::from_project(&app_project.surfaces))
        };
        let model = if app_project.models.is_empty() {
            None
        } else {
            Some(ModelRelated::from_project(&app_project.models))
        };
        let disp_data = if app_project.sars.is_empty() {
            None
        } else {
            Some(DispGeomRelated::from_project(&app_project.sars))
        };

        Self { project, surface, model, disp_data }
    }
}

#[derive(Debug)]
enum ProjectError {
    MissingField(TomlField),
    Project(Box<ProjectError>),
    Surface(Box<ProjectError>),
    Model(Box<ProjectError>),
    DispData(Box<ProjectError>),
    Datas(Box<ProjectError>),
}

#[derive(Debug)]
enum TomlField {
    Project,
    Surface,
    Model,
    DispData,
    Name,
    Note,
    DemFilePath,
    DemAzimuth,
    FilePath,
    Azimuth,
    Incidence,
    Datas,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProjectRelated {
    name: String,
    note: Option<String>,
    dem_file_name: String,
    dem_azimuth: Option<f32>,
}

impl ProjectRelated {
    fn from_project(project: &Project) -> Self {
        let name = project.name.to_string();
        let note = if project.note.is_empty() {
            None
        } else {
            Some(project.note.to_string())
        };
        let dem_file_name = String::from("dem.csv");
        let dem_azimuth = match &project.dem.section_geometry {
            Some(geom) => Some(geom.azimuth.clone()),
            None => None,
        };

        Self {name, note, dem_file_name, dem_azimuth}
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct SurfaceRelated {
    name: String,
    file_name: String,
}

impl SurfaceRelated {
    fn from_project(surfaces: &Vec<BundleSurface>) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..surfaces.len() {
            let name = surfaces[k].name.to_string();
            let file_name = "surface_".to_string() + (k + 1).to_string().as_str() + ".csv";
            let surface_related = Self { name, file_name };
            relateds.push(surface_related);
        }
        relateds
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelRelated {
    name: String,
    file_name: String,
}

impl ModelRelated {
    fn from_project(models: &Vec<BundleModel>) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..models.len() {
            let name = models[k].name.to_string();
            let file_name = "model_".to_string() + (k + 1).to_string().as_str() + ".csv";
            let model_related = Self {name, file_name};
            relateds.push(model_related);
        }
        relateds
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DispGeomRelated {
    name: String,
    azimuth: f32,
    incidence: f32,
    datas: Option<Vec<DispDataRelated>>
}

impl DispGeomRelated {
    fn from_project(sar: &Vec<BundleSar>) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..sar.len() {
            let name = sar[k].name.to_string();
            let azimuth = sar[k].sar_geometry.azimuth;
            let incidence = sar[k].sar_geometry.incidence;
            let datas = if sar[k].disp_data.is_empty() {
                None
            } else {
                Some(DispDataRelated::from_project(&sar[k].disp_data))
            };
            let geom_related = Self { name, azimuth, incidence, datas };
            relateds.push(geom_related);            
        }
        relateds
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DispDataRelated {
    name: String,
    file_name: String,
}

impl DispDataRelated {
    fn from_project(data: &Vec<BundleDispData>) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..data.len() {
            let name = data[k].name.to_string();
            let file_name = "data_".to_string() + (k + 1).to_string().as_str() + ".csv";
            let data_related = Self {name, file_name};
            relateds.push(data_related);
        }
        relateds
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use fs::File;

    use super::*;

    #[test]
    fn test_toml_empty() {
        let project = ProjectFile {
            project: ProjectRelated { name: String::from("A Sample Project"), note: None, dem_file_name: String::from("dem.csv"), dem_azimuth: None},
            surface: None,
            model: None,
            disp_data: None,
        };
        let toml = toml::to_string(&project).unwrap();
        let mut file = File::create("src-logic/test_data/project_files/empty_project.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    #[test]
    fn test_toml() {
        let proj = ProjectFile {
            project: ProjectRelated { name: String::from("My dear little project"), 
                note: Some(String::from("Usefull data I want to remember about my project")), 
                dem_file_name: String::from("dem.csv"),
                dem_azimuth: Some(289.),
            },
            surface: Some(vec![SurfaceRelated{name: String::from("surf1"), file_name: String::from("surf1.csv")},
            SurfaceRelated{name: String::from("surf2"), file_name: String::from("surf2.csv")}]),
            model: Some(vec![ModelRelated{name: String::from("model one"), file_name: String::from("model1.csv")},
            ModelRelated{name: String::from("model two"), file_name: String::from("model2.csv")},
            ModelRelated{name: String::from("model three"), file_name: String::from("model3.csv")}]),
            disp_data: Some(vec![DispGeomRelated{
                name: String::from("sat_geometry"),
                azimuth: 260.,
                incidence: 35.,
                datas: Some(vec![DispDataRelated{name: String::from("data one"), file_name: String::from("data1.csv")},
                DispDataRelated{name: String::from("data two"), file_name: String::from("data2.csv")}])
            },
            DispGeomRelated{
                name: String::from("sat_geometry2"),
                azimuth: 260.,
                incidence: 35.,
                datas: None,
            }]),
        };
        let toml = toml::to_string(&proj).unwrap();
        println!("toml:\n{}", toml);
        let mut file = fs::File::create("src-logic/test_data/project_files/project_file.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }
}