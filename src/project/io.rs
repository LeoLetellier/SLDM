use super::*;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::Path;
use toml;

impl Project {
    pub(crate) fn save(&self) -> Result<()> {
        // Assume that root was well defined by the user
        let root = match self.path.clone() {
            Some(p) => p,
            None => bail!("No saving path specified"),
        };

        save_toml(self, &root)?;
        save_all_csv(self, &root)?;
        Ok(())
    }

    pub(crate) fn load(&mut self, path: &String) -> Result<Project> {
        let path = match path {
            p if p.is_empty() => bail!("No saving path specified"),
            p => p.to_string(),
        };

        let (mut project, project_file) = load_toml(&path)?;
        load_all_csv(&mut project, &project_file)?;

        Ok(project)
    }
}

pub(crate) fn save_toml(project: &Project, root_folder: &String) -> Result<()> {
    let project_definition = ProjectFile::from_project(project);
    let toml = toml::to_string(&project_definition)?;

    let mut file = std::fs::File::create(root_folder.to_string() + "/project.toml")?;
    file.write_all(toml.as_bytes())?;

    Ok(())
}

fn load_toml(path: &String) -> Result<(Project, ProjectFile)> {
    let toml_contents = std::fs::read_to_string(path)?;
    let project_file: ProjectFile = toml::from_str(&toml_contents)?;
    let project = project_file.to_project(path);
    Ok((project, project_file))
}

pub(crate) fn save_all_csv(project: &Project, root_folder: &String) -> Result<()> {
    let path = root_folder.to_string() + "/dem.csv";
    if !project.dem.dem.x.is_empty() {
        project.dem.to_csv(&path)?;
    }

    for s in 0..project.surfaces.len() {
        let path = root_folder.to_string() + "/surface_" + (s + 1).to_string().as_str() + ".csv";
        project.surfaces[s].to_csv(&path, &project.dem.dem.x)?;
    }

    for m in 0..project.models.len() {
        let path = root_folder.to_string() + "/model_" + (m + 1).to_string().as_str() + ".csv";
        project.models[m].to_csv(&path, &project.dem.dem.x)?;
    }

    for g in 0..project.sars.len() {
        for d in 0..project.sars[g].disp_data.len() {
            let path = root_folder.to_string()
                + "/disp_"
                + (g + 1).to_string().as_str()
                + "_data_"
                + (d + 1).to_string().as_str()
                + ".csv";
            project.sars[g].disp_data[d].to_csv(&path)?;
        }
    }

    Ok(())
}

fn load_all_csv(project: &mut Project, project_file: &ProjectFile) -> Result<()> {
    let root = match &project.path {
        None => bail!("No root path"),
        Some(p) => p,
    };

    let root = Path::parent(Path::new(root));
    let root = match root {
        Some(path) => match path.to_str() {
            Some(path) => path.to_string() + "\\",
            None => bail!(""),
        },
        None => bail!("No parent folder. What have you done ?"),
    };

    let path = root.to_string() + project_file.project.dem_file_name.as_str();
    project.dem.from_csv(&path)?;

    match &project_file.surface {
        Some(surfaces) => {
            for s in 0..surfaces.len() {
                let path = root.to_string() + surfaces[s].file_name.to_string().as_str();
                project.surfaces[s].from_csv(&path, &project.dem.dem)?;
            }
        }
        None => (),
    }

    match &project_file.model {
        Some(models) => {
            for m in 0..models.len() {
                let path = root.to_string() + models[m].file_name.to_string().as_str();
                project.models[m].from_csv(&path, &project.dem.dem)?;
            }
        }
        None => (),
    }

    match &project_file.disp_data {
        Some(geoms) => {
            for g in 0..geoms.len() {
                match &geoms[g].datas {
                    Some(datas) => {
                        for d in 0..datas.len() {
                            let path = root.to_string() + datas[d].file_name.to_string().as_str();
                            project.sars[g].disp_data[d].from_csv(&path)?;
                        }
                    }
                    None => (),
                }
            }
        }
        None => (),
    }

    Ok(())
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

        Self {
            project,
            surface,
            model,
            disp_data,
        }
    }

    fn to_project(&self, root_folder: &String) -> Project {
        let mut project = ProjectRelated::to_project(&self.project, root_folder);

        match &self.surface {
            Some(s) => {
                for surface_related in s {
                    SurfaceRelated::to_project(&surface_related, &mut project);
                }
            }
            None => (),
        }

        match &self.model {
            Some(m) => {
                for model_related in m {
                    ModelRelated::to_project(&model_related, &mut project);
                }
            }
            None => (),
        }

        match &self.disp_data {
            Some(d) => {
                for disp_related in d {
                    DispGeomRelated::to_project(&disp_related, &mut project);
                }
            }
            None => (),
        }

        project
    }
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

        Self {
            name,
            note,
            dem_file_name,
            dem_azimuth,
        }
    }

    fn to_project(&self, root_folder: &String) -> Project {
        let name = self.name.to_string();
        let path = Some(root_folder.to_string());
        let note = match self.note.to_owned() {
            Some(n) => n,
            None => String::new(),
        };
        let mut dem = BundleDem::default();
        dem.section_geometry = match self.dem_azimuth {
            Some(az) => match Orientation::new(az, std::f32::consts::PI / 2.0) {
                Err(_) => None,
                Ok(o) => Some(o),
            },
            None => None,
        };

        Project {
            name,
            path,
            note,
            dem,
            surfaces: vec![],
            models: vec![],
            sars: vec![],
        }
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

    fn to_project(&self, project: &mut Project) {
        let mut bundle = BundleSurface::default();
        bundle.name = self.name.to_string();
        project.surfaces.push(bundle);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelRelated {
    name: String,
    file_name: String,
    weights: Vec<f32>,
    boundaries: Vec<(usize, usize)>,
    gradients: Vec<Vec<(usize, f32)>>,
}

impl ModelRelated {
    fn from_project(models: &Vec<BundleModel>) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..models.len() {
            let name = models[k].name.to_string();
            let file_name = "model_".to_string() + (k + 1).to_string().as_str() + ".csv";
            let weights = models[k].weights.clone();
            let boundaries = models[k].boundaries.clone();
            let gradients = models[k].gradients.clone();
            let model_related = Self {
                name,
                file_name,
                weights,
                boundaries,
                gradients,
            };
            relateds.push(model_related);
        }
        relateds
    }

    fn to_project(&self, project: &mut Project) {
        let mut bundle = BundleModel::default();
        bundle.name = self.name.to_string();
        bundle.weights = self.weights.clone();
        bundle.boundaries = self.boundaries.clone();
        bundle.gradients = self.gradients.clone();
        project.models.push(bundle);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DispGeomRelated {
    name: String,
    azimuth: f32,
    incidence: f32,
    datas: Option<Vec<DispDataRelated>>,
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
                Some(DispDataRelated::from_project(&sar[k].disp_data, k))
            };
            let geom_related = Self {
                name,
                azimuth,
                incidence,
                datas,
            };
            relateds.push(geom_related);
        }
        relateds
    }

    fn to_project(&self, project: &mut Project) {
        let mut bundle = BundleSar::default();
        bundle.name = self.name.to_string();
        bundle.sar_geometry = Orientation::new(self.azimuth, self.incidence).unwrap_or_default();
        let mut sub_bundles = vec![];
        match &self.datas {
            Some(data) => {
                for d in data {
                    sub_bundles.push(DispDataRelated::to_project(&d));
                }
            }
            None => (),
        }
        bundle.disp_data = sub_bundles;
        project.sars.push(bundle);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct DispDataRelated {
    name: String,
    file_name: String,
}

impl DispDataRelated {
    fn from_project(data: &Vec<BundleDispData>, geom_index: usize) -> Vec<Self> {
        let mut relateds = vec![];
        for k in 0..data.len() {
            let name = data[k].name.to_string();
            let file_name = "disp".to_string()
                + "_"
                + (geom_index + 1).to_string().as_str()
                + "_data_"
                + (k + 1).to_string().as_str()
                + ".csv";
            let data_related = Self { name, file_name };
            relateds.push(data_related);
        }
        relateds
    }

    fn to_project(&self) -> BundleDispData {
        let mut bundle = BundleDispData::default();
        bundle.name = self.name.to_string();
        bundle
    }
}

impl BundleDem {
    fn from_csv(&mut self, path: &String) -> Result<()> {
        let reader = CsvReader::read(path.to_string(), None)?;
        let datas = reader.get_datas(&vec!["x".to_string(), "z".to_string()])?;
        self.dem.x = datas[0].clone();
        self.dem.surface.z = datas[1].clone();
        Ok(())
    }

    fn to_csv(&self, path: &String) -> Result<()> {
        let datas = vec![self.dem.x.clone(), self.dem.surface.z.clone()];
        let headers = vec!["x".to_string(), "z".to_string()];
        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(path, None)?;
        Ok(())
    }
}

impl BundleSurface {
    fn from_csv(&mut self, path: &String, dem: &Dem1D) -> Result<()> {
        let reader = CsvReader::read(path.to_string(), None)?;
        let datas = reader.get_datas(&vec!["x".to_string(), "z".to_string()])?;
        let _x = datas[0].clone();
        // check if is not the same data as DEM TODO
        self.surface.z = datas[1].clone();
        let profile = DispProfile::from_surface_direct(&mut self.surface, dem)?;
        self.profile = profile;
        Ok(())
    }

    fn to_csv(&self, path: &String, x_dem: &Vec<f32>) -> Result<()> {
        let datas = vec![x_dem.clone(), self.surface.z.clone()];
        let headers = vec!["x".to_string(), "z".to_string()];
        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(path, None)?;
        Ok(())
    }

    pub(crate) fn export_values(&mut self, path: &String, dem: &Dem1D) -> Result<()> {
        if self.surface.slope.is_none() {
            self.surface.get_slope(dem);
        }
        let origins = self.profile.origins.clone();
        let vecs = self.profile.vecs.clone();

        let datas = vec![
            dem.x.clone(),
            self.surface.z.clone(),
            self.surface.slope.clone().unwrap(),
            origins.iter().map(|[a, _]| *a).collect(),
            origins.iter().map(|[_, b]| *b).collect(),
            vecs.iter().map(|v| v.coords().0).collect(),
            vecs.iter().map(|v| v.coords().1).collect(),
        ];

        let headers = vec![
            "x".to_string(),
            "z".to_string(),
            "slope".to_string(),
            "ox".to_string(),
            "oz".to_string(),
            "vx".to_string(),
            "vz".to_string(),
        ];

        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(path, None)?;
        Ok(())
    }
}

impl BundleModel {
    fn from_csv(&mut self, path: &String, dem: &Dem1D) -> Result<()> {
        let reader = CsvReader::read(path.to_string(), None)?;
        let nb_headers = reader.headers.len();
        let _x = reader.get_data(&"x".to_string())?;
        // check if is not the same data as DEM TODO
        for k in 0..(nb_headers - 1) {
            let z = reader.get_data(&("z".to_string() + k.to_string().as_str()))?;
            let surface = Surface1D::new(z);
            self.surfaces.push(surface);
        }
        let boundaries = self.boundaries.iter().map(|(a, b)| [*a, *b]).collect();
        self.resulting_profile = DispProfile::from_surfaces(
            dem,
            &mut self.surfaces,
            &boundaries,
            &self.gradients,
            &self.weights,
        )?;
        Ok(())
    }

    fn to_csv(&self, path: &String, x_dem: &Vec<f32>) -> Result<()> {
        let mut datas = vec![x_dem.clone()];
        let mut headers = vec!["x".to_string()];
        for k in 0..self.surfaces.len() {
            datas.push(self.surfaces[k].z.clone());
            headers.push("z".to_string() + k.to_string().as_str());
        }
        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(path, None)?;
        Ok(())
    }

    pub(crate) fn export_values(&self, path: &String, amp_los: &Vec<f32>) -> Result<()> {
        let origins = self.resulting_profile.origins.clone();
        let vecs = self.resulting_profile.vecs.clone();

        let mut datas = vec![
            origins.iter().map(|[a, _]| *a).collect(),
            origins.iter().map(|[_, b]| *b).collect(),
            vecs.iter().map(|v| v.coords().0).collect(),
            vecs.iter().map(|v| v.coords().1).collect(),
            vecs.iter().map(|v| v.angle_rad()).collect(),
            vecs.iter().map(|v| v.amplitude()).collect(),
        ];
        let mut headers = vec![
            "ox".to_string(),
            "oz".to_string(),
            "vx".to_string(),
            "vz".to_string(),
            "slope".to_string(),
            "amplitude".to_string(),
        ];

        if !amp_los.is_empty() {
            datas.push(amp_los.clone());
            headers.push("amp_in_los".to_string());
        }

        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(path, None)?;
        Ok(())
    }
}

impl BundleDispData {
    fn from_csv(&mut self, path: &String) -> Result<()> {
        let reader = CsvReader::read(path.clone(), None)?;
        let datas = reader.get_datas(&vec!["x".to_string(), "disp".to_string()])?;
        self.disp_data.x = datas[0].clone();
        self.disp_data.amplitude = datas[1].clone();
        Ok(())
    }

    fn to_csv(&self, path: &String) -> Result<()> {
        let datas = vec![self.disp_data.x.clone(), self.disp_data.amplitude.clone()];
        let headers = vec!["x".to_string(), "disp".to_string()];
        let writer = CsvWriter::from_datas_headers(datas, headers)?;
        writer.write(&path.clone(), None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_toml_empty() {
        let project = ProjectFile {
            project: ProjectRelated {
                name: String::from("A Sample Project"),
                note: None,
                dem_file_name: String::from("dem.csv"),
                dem_azimuth: None,
            },
            surface: None,
            model: None,
            disp_data: None,
        };
        let toml = toml::to_string(&project).unwrap();
        let mut file =
            std::fs::File::create("src-logic/test_data/project_files/empty_project.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }

    #[test]
    fn test_toml() {
        let proj = ProjectFile {
            project: ProjectRelated {
                name: String::from("My dear little project"),
                note: Some(String::from(
                    "Usefull data I want to remember about my project",
                )),
                dem_file_name: String::from("dem.csv"),
                dem_azimuth: Some(289.),
            },
            surface: Some(vec![
                SurfaceRelated {
                    name: String::from("surf1"),
                    file_name: String::from("surf1.csv"),
                },
                SurfaceRelated {
                    name: String::from("surf2"),
                    file_name: String::from("surf2.csv"),
                },
            ]),
            model: Some(vec![
                ModelRelated {
                    name: String::from("model one"),
                    file_name: String::from("model1.csv"),
                    weights: vec![],
                    boundaries: vec![],
                    gradients: vec![],
                },
                ModelRelated {
                    name: String::from("model two"),
                    file_name: String::from("model2.csv"),
                    weights: vec![],
                    boundaries: vec![],
                    gradients: vec![],
                },
                ModelRelated {
                    name: String::from("model three"),
                    file_name: String::from("model3.csv"),
                    weights: vec![],
                    boundaries: vec![],
                    gradients: vec![],
                },
            ]),
            disp_data: Some(vec![
                DispGeomRelated {
                    name: String::from("sat_geometry"),
                    azimuth: 260.,
                    incidence: 35.,
                    datas: Some(vec![
                        DispDataRelated {
                            name: String::from("data one"),
                            file_name: String::from("data1.csv"),
                        },
                        DispDataRelated {
                            name: String::from("data two"),
                            file_name: String::from("data2.csv"),
                        },
                    ]),
                },
                DispGeomRelated {
                    name: String::from("sat_geometry2"),
                    azimuth: 260.,
                    incidence: 35.,
                    datas: None,
                },
            ]),
        };
        let toml = toml::to_string(&proj).unwrap();
        println!("toml:\n{}", toml);
        let mut file =
            std::fs::File::create("src-logic/test_data/project_files/project_file.toml").unwrap();
        file.write_all(toml.as_bytes()).unwrap();
    }
}
