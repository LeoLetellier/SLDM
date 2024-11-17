pub(crate) mod io;

use src_logic::prelude::*;
use anyhow::{anyhow, Result};
use crate::components::command::SurfaceParams;


#[derive(Debug)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) path: Option<String>,
    pub(crate) note: String,

    pub(crate) dem: BundleDem,
    pub(crate) surfaces: Vec<BundleSurface>,
    pub(crate) models: Vec<BundleModel>,
    pub(crate) sars: Vec<BundleSar>,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: String::from("Unnamed Project"),
            path: None,
            note: String::new(),
            dem: BundleDem::default(),
            surfaces: vec![],
            models: vec![],
            sars: vec![],
        }
    }
}

impl Project {
    pub(crate) fn open_dem_from_file(&mut self, path: String) -> Result<()> {
        let reader = CsvReader::read(path, None)?;
        let dem = Dem1D::from_csv_reader(&reader, &mut String::new(), &mut String::new())?;
        self.dem.dem = dem;
        Ok(())
    }

    pub(crate) fn open_surface_from_file(&mut self, path: String, name: String) -> Result<()> {
        let reader = CsvReader::read(path, None)?;
        let mut surface = Surface1D::from_csv_reader(&reader, &self.dem.dem, &mut String::new(), &mut String::new())?;
        let profile = DispProfile::from_surface_direct(&mut surface, &self.dem.dem)?;
        let mut bundle = BundleSurface::default();
        bundle.name = name;
        bundle.surface = surface;
        bundle.profile = profile;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_exact_slbl(&mut self, first_pnt: usize, last_pnt: usize, tol: f32) -> Result<()> {
        let mut surface = Surface1D::from_slbl_exact(&self.dem.dem, first_pnt, last_pnt, tol);
        let profile = DispProfile::from_surface(&mut surface, &self.dem.dem, first_pnt, last_pnt)?;
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.profile = profile;
        bundle.name = String::from("SLBL_E_") + first_pnt.to_string().as_str() + "_" + last_pnt.to_string().as_str() + "_" + tol.to_string().as_str();
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_routine_slbl(&mut self, first_pnt: usize, last_pnt: usize, tol: f32, n_it: usize, elevation_min: f32, slope_max: f32) -> Result<()> {
        let mut surface = Surface1D::from_slbl_routine(&self.dem.dem, first_pnt, last_pnt, tol, n_it, Some(elevation_min), Some(slope_max));
        let profile = DispProfile::from_surface(&mut surface, &self.dem.dem, first_pnt, last_pnt)?;
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.profile = profile;
        bundle.name = String::from("SLBL_R_") + first_pnt.to_string().as_str() + "_" + last_pnt.to_string().as_str() + "_" + tol.to_string().as_str();
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_min(&mut self, surf1_index: usize, surf2_index: usize) -> Result<()> {
        let mut surface = Surface1D::from_min_surf(&self.surfaces[surf1_index].surface, &self.surfaces[surf2_index].surface);
        let profile = DispProfile::from_surface_direct(&mut surface, &self.dem.dem)?;
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.profile = profile;
        bundle.name = String::from("MIN_") + &self.surfaces[surf1_index].name + "_" + &self.surfaces[surf2_index].name;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_max(&mut self, surf1_index: usize, surf2_index: usize) -> Result<()> {
        let mut surface = Surface1D::from_max_surf(&self.surfaces[surf1_index].surface, &self.surfaces[surf2_index].surface);
        let profile = DispProfile::from_surface_direct(&mut surface, &self.dem.dem)?;
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.profile = profile;
        bundle.name = String::from("MAX_") + &self.surfaces[surf1_index].name + "_" + &self.surfaces[surf2_index].name;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn combine_unit_models(&mut self, name: &String, surface_params: &Vec<SurfaceParams>) -> Result<()> {
        let mut new_bundle = BundleModel::default();
        new_bundle.name = name.to_owned();
        let mut surfaces = vec![];
        let mut boundaries = vec![];
        let mut gradient = vec![];
        let mut weights = vec![];
        for s in 0..surface_params.len() {
            let param = &surface_params[s];
            new_bundle.surfaces.push(self.surfaces[param.index].surface.clone());
            new_bundle.weights.push(param.weight);
            new_bundle.boundaries.push(param.boundaries);
            new_bundle.gradients.push(param.gradient_points.to_owned());
            surfaces.push(self.surfaces[param.index].surface.clone());
            boundaries.push([param.boundaries.0, param.boundaries.1]);
            gradient.push(param.gradient_points.to_owned());
            weights.push(param.weight);
        }
        new_bundle.resulting_profile = DispProfile::from_surfaces(&self.dem.dem, &mut surfaces, &boundaries, &gradient, &weights)?;
        self.models.push(new_bundle);
        Ok(())
    }

    pub(crate) fn new_sar_data(&mut self, name: &String, sar_index: usize, file_path: String) -> Result<()> {
        let mut new_bundle = BundleDispData::default();
        let reader = CsvReader::read(file_path, None)?;
        new_bundle.name = name.to_owned();
        new_bundle.disp_data = DispData::from_csv_reader(&reader, &mut String::new(), &mut String::new())?;
        self.sars[sar_index].disp_data.push(new_bundle);
        Ok(())
    }

    pub(crate) fn calibrate_model(&mut self, model_index: usize, sar_index: usize, sar_data_index: usize) -> Result<()> {
        let model = &self.models[model_index];
        let sar_geom = &self.sars[sar_index];
        let sar_data = &sar_geom.disp_data[sar_data_index];
        let mut bundle = model.clone();
        let boundaries = model.boundaries.iter().map(|(a, b)| [*a, *b]).collect();
        let result = DispProfile::from_solver(&self.dem.dem, &model.surfaces, &boundaries, &model.gradients, &sar_data.disp_data, &self.dem.section_geometry.clone().unwrap(), &sar_geom.sar_geometry);

        match result {
            Ok((profile, weights)) => {
                bundle.weights = weights;
                bundle.resulting_profile = profile;
                bundle.name = "Calibrated_".to_string() + bundle.name.as_str();
                self.models.push(bundle);
                Ok(())
            }
            Err(e) => Err(anyhow!(e)),
        }
    }
}

#[derive(Debug)]
pub(crate) struct BundleDem {
    pub(crate) dem: Dem1D,
    pub(crate) section_geometry: Option<Orientation>,
    pub(crate) section_surface: bool,
    pub(crate) min_bound: [f64; 2],
    pub(crate) max_bound: [f64; 2],
}

impl Default for BundleDem {
    fn default() -> Self {
        BundleDem {
            dem: Dem1D::default(),
            section_geometry: None,
            section_surface: true,
            min_bound: [0., 0.],
            max_bound: [0., 0.]
        }
    }
}

#[derive(Debug)]
pub(crate) struct BundleSurface {
    pub(crate) name: String,
    pub(crate) surface: Surface1D,
    pub(crate) section_surface: bool,
    
    pub(crate) profile: DispProfile,
    pub(crate) section_arrow: bool,
    pub(crate) arrow_scaling_factor: f32,
    pub(crate) section_pillar: bool,
    pub(crate) color_surface: Option<egui::Color32>,
    pub(crate) color_arrow: Option<egui::Color32>,
}

impl Default for BundleSurface {
    fn default() -> Self {
        BundleSurface {
            name: String::new(),
            surface: Surface1D::default(),
            section_surface: true,
            profile: DispProfile::default(),
            section_arrow: false,
            arrow_scaling_factor: 1.0,
            section_pillar: false,
            color_surface: None,
            color_arrow: None,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct BundleModel {
    pub(crate) name: String,

    pub(crate) surfaces: Vec<Surface1D>,
    pub(crate) weights: Vec<f32>,
    pub(crate) boundaries: Vec<(usize, usize)>,
    pub(crate) gradients: Vec<Vec<(usize, f32)>>,

    pub(crate) resulting_profile: DispProfile,

    pub(crate) section_arrow: bool,
    pub(crate) arrow_scaling_factor: f32,
    pub(crate) arrow_color: Option<egui::Color32>,
}

impl Default for BundleModel {
    fn default() -> Self {
        BundleModel {
            name: String::from("model"),
            surfaces: vec![],
            weights: vec![],
            boundaries: vec![],
            gradients: vec![],
            resulting_profile: DispProfile::default(),
            section_arrow: true,
            arrow_scaling_factor: 10.0,
            arrow_color: None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BundleSar {
    pub(crate) name: String,
    pub(crate) sar_geometry: Orientation,
    pub(crate) disp_data: Vec<BundleDispData>,
}

#[derive(Debug, Default)]
pub(crate) struct BundleDispData {
    pub(crate) name: String,
    pub(crate) disp_data: DispData,
    pub(crate) section_arrow: bool,
    pub(crate) property_disp: bool,
}
