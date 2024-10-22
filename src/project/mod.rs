pub(crate) use src_logic::types;


#[derive(Debug, Default)]
pub(crate) struct Project {
    name: String,
    path: String,
    note: String,

    bundle_dem: BundleDem,
    bundler_surface: Vec<BundleSurface>,
    bundle_model: Vec<BundleModel>,
    bundle_sar: Vec<BundleSar>,
}

#[derive(Debug, Default)]
struct BundleDem {
    dem: types::Dem1D,
    section_geometry: Option<types::Orientation>,
}

#[derive(Debug, Default)]
struct BundleSurface {
    surface: types::Surface1D,
}

#[derive(Debug, Default)]
struct BundleModel {
    profile: types::DispProfile,
    unit: bool,
}

#[derive(Debug, Default)]
struct BundleSar {
    sar_geometry: types::Orientation,
    disp_data: Vec<types::DispData>,
}

