use src_logic::types::*;
use src_logic::io_csv::*;
use anyhow::Result;


#[derive(Debug)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) path: Option<String>,
    pub(crate) note: String,

    pub(crate) dem: BundleDem,
    pub(crate) surfaces: Vec<BundleSurface>,
    pub(crate) unit_models: Vec<BundleUnitModel>,
    pub(crate) composition_models: Vec<BundleCompositeModel>,
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
            unit_models: vec![],
            composition_models: vec![],
            sars: vec![],
        }
    }
}

impl Project {
    pub(crate) fn open_dem_from_file(&mut self, path: String) -> Result<()> {
        let dem = Dem1D::from_csv(path)?;
        self.dem.dem = dem;
        Ok(())
    }

    pub(crate) fn open_surface_from_file(&mut self, path: String, name: String) -> Result<()> {
        let surface = Surface1D::from_csv(path)?;
        let mut bundle = BundleSurface::default();
        bundle.name = name;
        bundle.surface = surface;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_exact_slbl(&mut self, first_pnt: usize, last_pnt: usize, tol: f32) -> Result<()> {
        let surface = Surface1D::from_slbl_exact(&self.dem.dem, first_pnt, last_pnt, tol);
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.name = String::from("SLBL_E_") + first_pnt.to_string().as_str() + "_" + last_pnt.to_string().as_str() + "_" + tol.to_string().as_str();
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_min(&mut self, surf1_index: usize, surf2_index: usize) -> Result<()> {
        let surface = Surface1D::from_min_surf(&self.surfaces[surf1_index].surface, &self.surfaces[surf2_index].surface);
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.name = String::from("MIN_") + &self.surfaces[surf1_index].name + "_" + &self.surfaces[surf2_index].name;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn surface_from_max(&mut self, surf1_index: usize, surf2_index: usize) -> Result<()> {
        let surface = Surface1D::from_max_surf(&self.surfaces[surf1_index].surface, &self.surfaces[surf2_index].surface);
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        bundle.name = String::from("MAX_") + &self.surfaces[surf1_index].name + "_" + &self.surfaces[surf2_index].name;
        self.surfaces.push(bundle);
        Ok(())
    }

    pub(crate) fn disp_from_surf(&mut self, surf_index: usize, first_pnt: usize, last_pnt: usize) -> Result<()> {
        let disp = DispProfile::from_surface(&mut self.surfaces[surf_index].surface, &self.dem.dem, first_pnt, last_pnt);
        let mut bundle = BundleUnitModel::default();
        bundle.name = self.surfaces[surf_index].name.clone() + "_" + first_pnt.to_string().as_str() + "_" + last_pnt.to_string().as_str();
        bundle.profile = disp;
        bundle.arrow_scaling_factor = 1.;
        self.unit_models.push(bundle);
        Ok(())
    }

    pub(crate) fn apply_model_gradient(&mut self, model_index: usize, gradient: &Vec<(usize, f32)>) -> Result<()> {
        self.unit_models[model_index].profile.apply_amplitude_gradient(gradient);
        Ok(())
    }
}

#[derive(Debug)]
pub(crate) struct BundleDem {
    pub(crate) dem: Dem1D,
    pub(crate) section_geometry: Option<Orientation>,
    pub(crate) section_surface: bool,
}

impl Default for BundleDem {
    fn default() -> Self {
        BundleDem {
            dem: Dem1D::default(),
            section_geometry: None,
            section_surface: true,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct BundleSurface {
    pub(crate) name: String,
    pub(crate) surface: Surface1D,
    pub(crate) section_surface: bool,
}

#[derive(Debug, Default)]
pub(crate) struct BundleUnitModel {
    pub(crate) name: String,
    pub(crate) profile: DispProfile,
    pub(crate) section_arrow: bool,
    pub(crate) arrow_scaling_factor: f32,
    pub(crate) section_pillar: bool,
    pub(crate) property_disp: bool,
    pub(crate) property_proj_disp: bool,
}

#[derive(Debug, Default)]
pub(crate) struct BundleCompositeModel {
    pub(crate) name: String,
    pub(crate) profiles: Vec<DispProfile>,
    pub(crate) weights: Vec<f32>,
    pub(crate) section_arrow: bool,
    pub(crate) arrow_scaling_factor: f32,
    pub(crate) property_disp: bool,
    pub(crate) property_proj_disp: bool,
}

#[derive(Debug, Default)]
pub(crate) struct BundleSar {
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
