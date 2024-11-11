//! This module defines the types used to hold the information in this project.

use std::f32::consts::PI;
use crate::data::vec_proj::{deg2rad, Vector2Rep, Vector3Rep};
use thiserror::Error;

/// The 1D Digital Elevation Model representation.
/// 
/// It is one dimensional as it holds information in one axis only.
/// 
/// It defines the x sampling for all layers, the azimuth of 
/// the 2D section (increasing x-axis), and the elevation value
/// of th topography at each sampling points.
#[derive(Default, Debug, Clone)]
pub struct Dem1D {
    /// Orientation of the 2D section
    /// 
    /// Only the azimuth field should matter
    pub orientation: Option<Orientation>,
    /// Sampling values for the layers    
    pub x: Vec<f32>,
    /// Elevation value of the topography at each sampling points
    pub surface: Surface1D,
}

#[derive(Debug, Error)]
pub enum VectorInputError {
    #[error("Length of first ({vec1}) and second ({vec2}) vectors are not the same")]
    InconsistentLen {
        vec1: usize,
        vec2: usize,
    },
    #[error("Input vectors are empty")]
    EmptyVecs,
}

impl Dem1D {
    /// Construct a new Dem1D from the 2D section orientation and the x/z data
    /// 
    /// # Errors
    /// * the length of the x and z vectors differ
    /// * the x and z vectors are empty
    pub(crate) fn new(x: Vec<f32>, z: Vec<f32>) -> Result<Self, VectorInputError> {
        if x.len() != z.len() {
            Err(VectorInputError::InconsistentLen { vec1: x.len(), vec2: z.len() })
        } else if x.len() == 0 {
            Err(VectorInputError::EmptyVecs)
        } else {
            let surface = Surface1D::new(z);
            Ok(Dem1D {orientation: None, x, surface})
        }
    }

    pub fn with_orientation(&mut self, orientation: Orientation) {
        self.orientation = Some(orientation);
    }
}

/// A 1D surface layer defining a surface along a dem sampling on a 2D section.
#[derive(Default, Debug, Clone)]
pub struct Surface1D {
    /// Elevation property
    pub z: Vec<f32>,
    /// The slope (centered) of the elevation property
    /// 
    /// Need to be computed before using
    pub slope: Option<Vec<f32>>,
}

impl Surface1D {
    /// Construct a new Surface1D from elevation values
    pub fn new(z: Vec<f32>) -> Self {
        Surface1D { z, slope: None }
    }
}

/// A 1D profile defining vectors and their positions on the 2D section
/// 
/// The defined vectors represents the ground displacement at origin points
/// 
/// The pillars representing the migration of the vectors from the failure surface
/// to the ground can be represented by segments from the failure surface sampling
/// to the said vectors origin points
#[derive(Default, Debug, Clone)]
pub struct DispProfile {
    /// 2D vector defined by its x and y components
    pub vecs: Vec<Vector2Rep>,
    /// Origin points of the 2D vectors
    pub origins: Vec<[f32; 2]>,
}

impl DispProfile {
    /// Construct a new vector profile from its vectors and their origins
    /// 
    /// # Errors
    /// * the length of the vecs and origins vectors differ
    pub fn new(vecs: Vec<Vector2Rep>, origins: Vec<[f32; 2]>) -> Result<Self, VectorInputError> {
        if vecs.len() != origins.len() {
            Err(VectorInputError::InconsistentLen { vec1: vecs.len(), vec2: origins.len() })
        } else {
            Ok(DispProfile { vecs, origins })
        }
    }

    pub fn from_slope_params(slope: Vec<f32>, amplitude: Vec<f32>, ox: Vec<f32>, oz: Vec<f32>, is_facing_right: bool) -> Result<Self, VectorInputError> {
        let mut vecs: Vec<Vector2Rep> = vec![];
        let mut origins: Vec<[f32; 2]> = vec![];

        for k in 0..slope.len() {
            let mut vec = Vector2Rep::from_deg(slope[k], is_facing_right);
            vec.with_norm(amplitude[k]);
            let origin = [ox[k], oz[k]];
            vecs.push(vec);
            origins.push(origin);
        }

        DispProfile::new(vecs, origins)
    }
}

/// A 1D displacement data profile
/// 
/// Contains displacement values along the 2D profile at positions that
/// may differs from the dem sampling
/// 
/// Must be associated with the acquisition orientation
#[derive(Default, Debug)]
pub struct DispData {
    /// x samples where displacement data were recorded
    pub x: Vec<f32>,
    /// amplitude of the displacement
    pub amplitude: Vec<f32>,
    /// projection of the recorded displacement into a section
    /// 
    /// Can be used to display the displacement into a 2D section.
    /// Not projected if empty
    pub projected_vecs: Vec<Vector2Rep>,
}

impl DispData {
    /// Construct a new displacement data profile from x / amplitude values
    pub fn new(x: Vec<f32>, amplitude: Vec<f32>) -> Result<Self, VectorInputError> {
        if x.len() != amplitude.len() {
            Err(VectorInputError::InconsistentLen { vec1: x.len(), vec2: amplitude.len() })
        } else if x.len() == 0 {
            Err(VectorInputError::EmptyVecs)
        } else {
            Ok(DispData { x, amplitude, projected_vecs: Vec::<Vector2Rep>::new() })
        }
    }
}

/// The orientation parametrization of satellite acquisition with azimuth and incidence angles.
#[derive(Debug, Default, PartialEq, Clone)]
pub struct Orientation {
    /// Azimuth angle
    /// 
    /// Represent the clockwise angle between the North and the azimuth of the acquisition sensor
    pub azimuth : f32,
    /// Incidence angle
    /// 
    /// Represent the angle between the perpendicular projection to the ground and the actual
    /// sensor inclination
    pub incidence : f32,
}

impl Into<Vector3Rep> for Orientation {
    /// Convert an orientation geometry into a vec3 Vector3Rep
    fn into(self) -> Vector3Rep {
        Vector3Rep::from_rad(self.azimuth, self.incidence - PI / 2.)
    }
}

#[derive(Debug, Error)]
pub enum OrientationError {
    #[error("The azimuth value {0} is out of range")]
    AzimuthOutOfRange(f32),
    #[error("The incidence value {0} is out of range")]
    IncidenceOutOfRange(f32),
}

impl Orientation {
    /// Construct a new orientation based on azimuth and incidence angles
    /// 
    /// # Errors
    /// * the azimuth or incidence value is out of range
    pub fn new(azimuth: f32, incidence: f32) -> Result<Self, OrientationError> {
        if !check_azimuth_range(azimuth) {
            Err(OrientationError::AzimuthOutOfRange(azimuth))
        } else if !check_incidence_range(incidence) {
            Err(OrientationError::IncidenceOutOfRange(incidence))
        } else {
            Ok(Orientation { azimuth, incidence })
        }
    }

    pub fn from_deg(azimuth: f32, incidence: f32) -> Result<Self, OrientationError> {
        let azimuth_rad = deg2rad(azimuth);
        let incidence_rad = deg2rad(incidence);
        Self::new(azimuth_rad, incidence_rad)
    }
}

/// Check the range of the azimuth value.
/// 
/// False if out of range
fn check_azimuth_range(azimuth: f32) -> bool {
    match azimuth {
        s if s < 0. => false,
        s if s >= 2. * PI => false,
        _ => true,
    }
}

/// Check the range of the incidence value.
/// 
/// False if out of range
fn check_incidence_range(incidence: f32) -> bool {
    match incidence {
        d if d < 0. => false,
        d if d > PI => false,
        _ => true,
    }
}
