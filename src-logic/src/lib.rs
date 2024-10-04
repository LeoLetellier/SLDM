//! Simple estimation of landslide's failure surface and displacement in 2D section.
//! 
//! Via a project manager, implement loading and writing csv inputs/outputs for properties along a section, 
//! generate a failure surface with SLBL method, simple unit displacement along combination of multiple 
//! failure surfaces. Compares the profile of displacement with SAR data, or other displacement measures
//! located on the section.

pub mod project;
pub mod slide;
pub mod types;
pub mod model;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
