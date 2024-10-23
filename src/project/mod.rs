use src_logic::types::*;
use std::sync::{Arc, Mutex};
use src_logic::io_csv::*;


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

#[derive(Debug, Default)]
pub(crate) struct BundleDem {
    dem: Dem1D,
    section_geometry: Option<Orientation>,
}

impl BundleDem {
    pub(crate) fn open_from_file(&mut self, path: String) {
        let dem = Dem1D::from_csv(path);
        self.dem = dem;
    }
}

#[derive(Debug, Default)]
struct BundleSurface {
    name: String,
    surface: Surface1D,
}

#[derive(Debug, Default)]
struct BundleUnitModel {
    name: String,
    profile: DispProfile,
}

#[derive(Debug, Default)]
struct BundleCompositeModel {
    name: String,
    profiles: Vec<Arc<DispProfile>>,
    weights: Arc<Vec<f32>>,
}

#[derive(Debug, Default)]
struct BundleSar {
    sar_geometry: Orientation,
    disp_data: Vec<BundleDispData>,
}

#[derive(Debug, Default)]
struct BundleDispData {
    name: String,
    disp_data: DispData,
}

