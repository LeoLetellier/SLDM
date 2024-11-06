use src_logic::types::*;
use src_logic::io_csv::*;
use anyhow::Result;


#[derive(Debug, Default)]
pub(crate) struct Project {
    pub(crate) name: String,
    pub(crate) path: String,
    pub(crate) note: String,

    pub(crate) dem: BundleDem,
    pub(crate) surfaces: Vec<BundleSurface>,
    pub(crate) unit_models: Vec<BundleUnitModel>,
    pub(crate) composition_models: Vec<BundleCompositeModel>,
    pub(crate) sars: Vec<BundleSar>,
}

impl Project {
    pub(crate) fn open_dem_from_file(&mut self, path: String) -> Result<()> {
        let dem = Dem1D::from_csv(path)?;
        self.dem.dem = dem;
        Ok(())
    }

    pub(crate) fn open_surface_from_file(&mut self, path: String) -> Result<()> {
        let surface = Surface1D::from_csv(path)?;
        let mut bundle = BundleSurface::default();
        bundle.surface = surface;
        self.surfaces.push(bundle);
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
