//! This module define how the vec2 and vec3 of this project are structured.
//! They are defined to perform cross-section operation using a vec2, 
//! and handling projection into a line of sight in the global 3D domain.
//! 
//! The vecs store their axis components, and also handles their angle definition.

use std::{f32::consts::PI, fmt::Display};
use assert_approx_eq::assert_approx_eq;
use std::ops::Mul;


/// Vector2Rep wrap all functionnalities to represent 2D vectors in cartesian
/// or spherical coordinates.
/// 
/// Data actually stored are:
/// * x-axis component
/// * y-axis component
/// 
/// Angle definition is defined from:
/// * slope: [-PI/2, PI/2] | [-90, 90]
/// * orientation: facing right, bool
/// 
/// The struct can be initialized from (x, y) using the `new` method, 
/// or from angles using `from_rad` or `from_deg` accordingly. Note that
/// a vector defined from an angle will be a unit vector that can then be 
/// normed usig `with_norm`.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2Rep {
    /// first component
    x: f32,
    /// second component
    y: f32,
}

impl Display for Vector2Rep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {}, y: {}, amp: {}, slope(rad|deg): {}|{}, facing_right: {}",
         self.x, self.y, self.amplitude(), self.angle_rad(), self.angle_deg(), self.is_facing_right())
    }
}

impl Vector2Rep {
    /// Construct a new vector using its cartesian components
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn coords(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    pub fn with_coords(&mut self, x: f32, y: f32) -> &Self {
        self.x = x;
        self.y = y;
        self
    }

    /// Get the angle of the vector in radians
    pub fn angle_rad(&self) -> f32 {
        if self.x != 0. {
            (self.y / self.x).atan()
        } else if self.y >= 0. {
                PI / 2.
        } else {
                - PI / 2.
        }
    }

    /// Get the angle of the vector in degrees
    pub fn angle_deg(&self) -> f32 {
        rad2deg(self.angle_rad())
    }


    /// Construct a new **unit** vector from an angle in radians
    /// and its orientation
    pub fn from_rad(slope: f32, is_facing_right: bool) -> Self {
        assert!(slope >= -PI / 2.);
        assert!(slope <= PI / 2.);

        let x_sign = if is_facing_right {1.0} else {-1.0};
        let (vx, vy) = match slope {
            s if s == PI / 2. => (0., 1.),
            s if s == - PI / 2. => (0., -1.),
            s => (x_sign * s.cos().abs(), s.sin()),
        };

        Vector2Rep::new(vx, vy)
    }

    /// Construct a new **unit** vector from an angle in degrees
    /// and its orientation
    pub fn from_deg(slope: f32, is_facing_right: bool) -> Self {
        let slope_rad = deg2rad(slope);
        Vector2Rep::from_rad(slope_rad, is_facing_right)
    }

    /// Get the amplitude of the vector
    pub fn amplitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// True if the vector is facing right considering the x-axis or null, False otherwise.
    pub fn is_facing_right(&self) -> bool {
        if self.x >= 0. {
            return true;
        }
        false
    }

    /// True if the vector is facing down considering the y-axis, false otherwise.
    pub fn is_facing_down(&self) -> bool {
        if self.y < 0. {
            return true;
        }
        false
    }

    /// Get the components of the corresponding unit vector
    pub fn get_unit(&self) -> (f32, f32) {
        let amp  = self.amplitude();
        if amp != 0. {
            (self.x / amp, self.y / amp)
        } else {
            (0., 0.)
        }
    }

    /// Transform the current vector into a unit vector
    pub fn unit(&mut self) -> &Self {
        (self.x, self.y) = self.get_unit();
        self
    }

    /// Transform the current vector to match a given norm 
    pub fn with_norm(&mut self, norm: f32) -> &Self {
        let (x, y) = self.get_unit();
        if norm != 0. {
            self.x = x * norm;
            self.y = y * norm;
        } else {
            self.x = 0.;
            self.y = 0.;
        }
        self
    }

    pub fn multiply(&mut self, factor: f32) -> &Self {
        self.x *= factor;
        self.y *= factor;
        self
    }
}

#[cfg(test)]
mod tests_vec2 {
    use super::*;

    #[test]
    fn test_vector2_simple() {
        let vec = Vector2Rep::new(1., 1.);
        println!("x: {}, y: {}, rad: {}, deg: {}, amp: {}, right: {}, down: {}", vec.x, vec.y, vec.angle_rad(), vec.angle_deg(), vec.amplitude(), vec.is_facing_right(), vec.is_facing_down());
        assert_eq!(vec.angle_rad(), 1.0_f32.atan());
        assert_eq!(vec.amplitude(), 2.0_f32.sqrt());
        assert!(vec.is_facing_right());
        assert!(!vec.is_facing_down());
    }

    #[test]
    fn test_up() {
        let vec = Vector2Rep::new(0., 1.);
        assert_eq!(vec.angle_rad(), PI / 2.);
        assert_eq!(vec.amplitude(), 1.);
        assert!(!vec.is_facing_down());
    }

    #[test]
    fn test_down() {
        let vec = Vector2Rep::new(0., -1.);
        assert_eq!(vec.angle_rad(), - PI / 2.);
        assert_eq!(vec.amplitude(), 1.);
        assert!(vec.is_facing_down());
    }
    

    #[test]
    fn test_left() {
        let vec = Vector2Rep::new(1., 0.);
        assert_eq!(vec.angle_rad(), 0.);
        assert_eq!(vec.amplitude(), 1.);
        assert!(vec.is_facing_right());
    }
    

    #[test]
    fn test_right() {
        let vec = Vector2Rep::new(-1., 0.);
        assert_eq!(vec.angle_rad(), 0.);
        assert_eq!(vec.amplitude(), 1.);
        assert!(!vec.is_facing_right());
    }

    #[test]
    fn test_from_angle_right() {
        let vec = Vector2Rep::from_deg(0.0, true);
        assert_eq!((vec.x, vec.y), (1.0, 0.0));
    }

    #[test]
    fn test_from_angle_left() {
        let vec = Vector2Rep::from_deg(0.0, false);
        assert_eq!((vec.x, vec.y), (-1.0, 0.0));
    }

    #[test]
    fn test_from_angle_up() {
        let vec = Vector2Rep::from_deg(90., true);
        assert_eq!((vec.x, vec.y), (0.0, 1.0));
    }

    #[test]
    fn test_from_angle_down() {
        let vec = Vector2Rep::from_deg(-90., true);
        assert_eq!((vec.x, vec.y), (0.0, -1.0));
    }

    #[test]
    fn test_from_angle_diag() {
        let vec = Vector2Rep::from_deg(-45., false);
        assert_eq!((vec.x, vec.y), (-1. / (2.0_f32.sqrt()), -1. / (2.0_f32.sqrt())));
    }

    #[test]
    fn from_all_angle() {
        let slope: Vec<f32> = (-9..=9).map(|k| (k * 10) as f32).collect();
        for s in &slope {
            let vec = Vector2Rep::from_deg(*s, true);
            assert_approx_eq!(vec.amplitude(), 1.);
            println!("slope {}/{}, x: {}, y: {}", s, true, vec.x, vec.y);
        }
        for s in &slope {
            let vec = Vector2Rep::from_deg(*s, false);
            assert_approx_eq!(vec.amplitude(), 1.);
            println!("slope {}/{}, x: {}, y: {}", s, false, vec.x, vec.y);
        }
    }
}

/// Vector3Rep wrap all functionnalities to represent 3D vectors in cartesian
/// or spherical coordinates.
/// 
/// Data actually stored are:
/// * x-axis component
/// * y-axis component
/// * z-axis component
/// 
/// Angle definition is defined from:
/// * azimuth: [0, 2 PI[ | [0, 360[
/// * dip: [-PI/2, PI/2] | [-90, 90]
/// 
/// The struct can be initialized from (x, y, z) using the `new` method, 
/// or from angles using `from_rad` or `from_deg` accordingly. Note that
/// a vector defined from an angle will be a unit vector that can then be 
/// normed usig `with_norm`.
/// 
/// A vec3 can be derived from a vec2 and the according section azimuth 
/// containing the vec2 using `from_vertical_section_rad` or 
/// `from_vertical_section_deg`.
/// 
/// The projection can be used with the `inner_product`, or with the resulting
/// projection vector using `project_onto`.
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector3Rep {
    /// first component
    x: f32,
    /// second component
    y: f32,
    /// third component
    z: f32,
}

impl Display for Vector3Rep {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let angles_deg = self.angle_deg();
        let angles_rad = self.angle_rad();
        write!(f, "x: {}, y: {}, z: {}, amp: {}, azimuth(rad|deg): {}|{}, dip(rad|deg): {}|{}",
         self.x, self.y, self.z, self.amplitude(), angles_rad.0, angles_deg.0, angles_rad.1, angles_deg.1)
    }
}

impl Vector3Rep {
    /// Construct a new vector using its cartesian components
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3Rep { x, y, z }
    }

    pub fn coords(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn with_coords(&mut self, x: f32, y: f32, z: f32) -> &Self {
        self.x = x;
        self.y = y;
        self.z = z;
        self
    }

    /// Get the spherical angles of the vector in radians
    pub fn angle_rad(&self) -> (f32, f32) {
        let (ux, uy, uz) = self.get_unit();

        let mut azimuth = - (ux / uy).atan();
        match (ux, uy) {
            (_vx, vy) if (vy < 0.) => azimuth += 3. * PI / 2.,
            (_vx, vy) if (vy > 0.) => azimuth += PI / 2.,
            (vx, vy) if (vy == 0.) & (vx < 0.) => azimuth = PI,
            _ => azimuth = 0.,
        };

        let dip = uz.asin();

        (azimuth, dip)
    }

    /// Get the spherical angles of the vector in degrees
    pub fn angle_deg(&self) -> (f32, f32) {
        let (azimuth, dip) = self.angle_rad();
        (rad2deg(azimuth), rad2deg(dip))
    }

    /// Construct a new **unit** vector from its spherical definition in radians
    pub fn from_rad(azimuth: f32, dip: f32) -> Self {
        let mut vx = azimuth.cos() * dip.cos();
        let mut vy = azimuth.sin() * dip.cos();
        let vz = dip.sin();

        if (dip == PI / 2.) | (dip == - PI / 2.) {
            (vx, vy) = (0., 0.);
        }

        Vector3Rep { x: vx, y: vy, z: vz }
    }

    /// Construct a new **unit** vector from its spherical definition in degrees
    pub fn from_deg(azimuth: f32, dip: f32) -> Self {
        let azimuth_rad = deg2rad(azimuth);
        let dip_rad = deg2rad(dip);

        Self::from_rad(azimuth_rad, dip_rad)
    }

    /// Get the amplitude of the vector
    pub fn amplitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    /// Get the components of the corresponding unit vector
    pub fn get_unit(&self) -> (f32, f32, f32) {
        let amp = self.amplitude();
        if amp != 0. {
            (self.x / amp, self.y / amp, self.z / amp)
        } else {
            (0., 0., 0.)
        }
    }

    /// Transform the current vector into a unit vector
    pub fn unit(&mut self) -> &Self {
        (self.x, self.y, self.z) = self.get_unit();
        self
    }

    /// Transform the current vector to match a given norm 
    pub fn with_norm(&mut self, norm: f32) -> &Self {
        let (x, y, z) = self.get_unit();
        if norm != 0. {
            self.x = x * norm;
            self.y = y * norm;
            self.z = z * norm;
        } else {
            self.x = 0.;
            self.y = 0.;
            self.z = 0.;
        }
        self
    }

    pub fn multiply(&mut self, factor: f32) -> &Self {
        self.x *= factor;
        self.y *= factor;
        self.z *= factor;
        self
    }

    /// Perform the inner product between the current vector and another
    pub fn inner_product(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Construct a new 3D vector from a local 2D vector and the according 
    /// 2D section azimuth in radians (orientation of the increasing x_axis)
    pub fn from_vertical_section_rad(section_vec: &Vector2Rep, azimuth: f32) -> Self {
        let Vector2Rep {
            x: v2x,
            y: v2y
        } = section_vec;

        // Vertical component is conserved
        let v3z = *v2y;

        // Horizontal component is distributed between x and y
        let v3x = *v2x * azimuth.cos();
        let v3y = *v2x * azimuth.sin();

        Vector3Rep::new(v3x, v3y, v3z)
    }

    /// Construct a new 3D vector from a local 2D vector and the according 
    /// 2D section azimuth in degrees (orientation of the increasing x_axis)
    pub fn from_vertical_section_deg(section_vec: &Vector2Rep, azimuth: f32) -> Self {
        let azimuth_rad = deg2rad(azimuth);
        Self::from_vertical_section_rad(section_vec, azimuth_rad)
    }

    /// Perform the projection of one vector onto another. 
    /// 
    /// Origin and Target vectors are not interchangeable
    pub fn project_onto(&self, other: &Self) -> Self {
        let mut projection_vector = *other.clone().unit();
        let projected_amplitude = self.inner_product(&projection_vector);
        projection_vector.with_norm(projected_amplitude);
        projection_vector
    }
}

#[cfg(test)]
mod test_vec3 {
    use super::*;

    #[test]
    fn all() {
        let az: Vec<f32> = (0..36).map(|k| (k * 10) as f32).collect();
        let dip: Vec<f32> = (-9..=9).map(|k| (k * 10) as f32).collect();
        for a in &az {
            for d in &dip {
                let vec = Vector3Rep::from_deg(*a, *d);
                assert_approx_eq!(vec.amplitude(), 1.0);
                println!("az: {}, d: {}, x: {}, y: {}, z: {}, amp: {}", a, d, vec.x, vec.y, vec.z, vec.amplitude());
                println!("az: {}, d: {}", vec.angle_deg().0, vec.angle_deg().1);
            }
        }
    }

    #[test]
    fn test_vector3_simple() {
        let vec = Vector3Rep::new(1. / 2.0_f32.sqrt(), 1. / 2.0_f32.sqrt(), 1.);
        let (azimuth, dip) = vec.angle_rad();
        assert_approx_eq!(azimuth, PI / 4.);
        assert_approx_eq!(dip, PI / 4.);
        println!("x: {}, y: {}, z: {}, rad: {}|{}, deg: {}|{}", vec.x, vec.y, vec.z, vec.angle_rad().0, vec.angle_rad().1, vec.angle_deg().0, vec.angle_deg().1);
    }

    #[test]
    fn test_az_up() {
        let vec = Vector3Rep::new(1., 0., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 0.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_az_down() {
        let vec = Vector3Rep::new(-1., 0., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, PI);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_az_right() {
        let vec = Vector3Rep::new(0., 1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, PI / 2.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_az_left() {
        let vec = Vector3Rep::new(0., -1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 3. * PI / 2.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_dip_up() {
        let vec = Vector3Rep::new(0., 0., 1.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 0.);
        assert_approx_eq!(angles.1, PI / 2.);
    }

    #[test]
    fn test_dip_down() {
        let vec = Vector3Rep::new(0., 0., -1.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 0.);
        assert_approx_eq!(angles.1, - PI / 2.);
    }

    #[test]
    fn test_quarter_up_right() {
        let vec = Vector3Rep::new(1., 1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, PI / 4.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_quarter_down_right() {
        let vec = Vector3Rep::new(-1., 1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 3. * PI / 4.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_quarter_down_left() {
        let vec = Vector3Rep::new(-1., -1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 5. * PI / 4.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_quarter_up_left() {
        let vec = Vector3Rep::new(1., -1., 0.);
        let angles = vec.angle_rad();
        assert_approx_eq!(angles.0, 7. * PI / 4.);
        assert_approx_eq!(angles.1, 0.);
    }

    #[test]
    fn test_from_vec2() {
        let vec2 = Vector2Rep::new(3.0, 2.0);
        let vec3 = Vector3Rep::from_vertical_section_deg(&vec2, 45.);
        println!("{}", vec2);
        println!("{}", vec3);
        assert_approx_eq!(vec3.x, 3. / 2.0_f32.sqrt());
        assert_approx_eq!(vec3.y, 3. / 2.0_f32.sqrt());
    }

    #[test]
    fn test_projection() {
        let vec_sar = Vector3Rep::from_deg(45., 35.);
        let vec_section = Vector2Rep::new(5., 1.);
        let vec_global = Vector3Rep::from_vertical_section_deg(&vec_section, 65.);
        println!("{vec_sar}");
        println!("{vec_section}");
        println!("{vec_global}");
        let proj = vec_global.project_onto(&vec_sar);
        println!("{}", vec_global.inner_product(&vec_sar));
        println!("{proj}");
        assert_approx_eq!(vec_global.inner_product(&vec_sar), proj.amplitude());
    }

    #[test]
    fn test_projection2() {
        let vec_sar = Vector3Rep::from_deg(50., 35.);
        let vec_section = Vector2Rep::new(5., 1.);
        for azimuth in (0..36).map(|k| (k * 10) as f32) {
            let vec_global = Vector3Rep::from_vertical_section_deg(&vec_section, azimuth);
            let proj = vec_global.project_onto(&vec_sar);
            println!("azimuth: {}, projected amplitude: {}", azimuth, vec_global.inner_product(&vec_sar).abs());
            assert_approx_eq!(vec_global.inner_product(&vec_sar).abs(), proj.amplitude());
        }
    }

}

/// Transform an f32 radians angle into an f32 degrees angle
pub fn rad2deg(rad: f32) -> f32 {
    rad * 180. / PI
}

/// Transform an f32 degrees angle into an f32 radians angle
pub fn deg2rad(deg: f32) -> f32 {
    deg * PI / 180.
} 
