//! Simple estimation of landslide's failure surface and displacement in 2D section.
//! 
//! Via a project manager, implement loading and writing csv inputs/outputs for properties along a section, 
//! generate a failure surface with SLBL method, simple unit displacement along combination of multiple 
//! failure surfaces. Compares the profile of displacement with SAR data, or other displacement measures
//! located on the section.

pub mod slide;
pub mod types;
pub mod io_csv;
pub mod profile;
pub mod data;
// pub mod plotter;

pub mod prelude {
    pub use crate::types::{Dem1D, Surface1D, DispProfile, DispData, Orientation};
    pub use crate::data::vec_proj::{Vector2Rep, Vector3Rep, deg2rad, rad2deg};
    pub use crate::io_csv::CSVReader;
}
