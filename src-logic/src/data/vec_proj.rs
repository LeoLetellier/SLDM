use std::f32::consts::PI;
use assert_approx_eq::assert_approx_eq;

/// VectorRep wrap all functionnalities to represent vectors at the same time
/// as (x,y) components and with angle / amplitude description.
/// 
/// Data actually stored are the (vx, vy) components. All additionnal data is 
/// derived from this.
/// 
/// The angle is expressed as a slope ranging from -PI/2 to PI/2. Horizontal is 
/// 0 rad, up is PI/2 rad, down is -PI/2 rad. Additionnal marker is needed to 
/// determine if the vector is facing right or left (x-axis).
#[derive(Debug, Default, Clone, Copy)]
pub struct Vector2Rep {
    /// x component
    vx: f32,
    /// y component
    vy: f32,
}

impl Vector2Rep {
    /// Construct a new vector using its components
    pub fn new(vx: f32, vy: f32) -> Self {
        Self { vx, vy }
    }

    /// Get the angle of the vector in radians
    pub fn angle_rad(&self) -> f32 {
        if self.vx != 0. {
            (self.vy / self.vx).atan()
        } else {
            if self.vy >= 0. {
                PI / 2.
            } else {
                - PI / 2.
            }
        }
    }

    /// Get the angle of the vector in degrees
    /// 
    /// i.e. orientation of the arrow
    pub fn angle_deg(&self) -> f32 {
        rad2deg(self.angle_rad())
    }

    pub fn from_rad(slope: f32, is_facing_right: bool) -> Self {
        assert!(slope >= -PI / 2.);
        assert!(slope <= PI / 2.);
        todo!()
    }

    /// Get the amplitude of the vector
    /// 
    /// i.e. length of the arrow
    pub fn amplitude(&self) -> f32 {
        (self.vx * self.vx + self.vy * self.vy).sqrt()
    }

    /// True if the vector is facing right considering the x-axis or null, False otherwise.
    pub fn is_facing_right(&self) -> bool {
        if self.vx >= 0. {
            return true;
        }
        false
    }

    /// True if the vector is facing down considering the y-axis, false otherwise.
    pub fn is_facing_down(&self) -> bool {
        if self.vy < 0. {
            return true;
        }
        false
    }

    pub fn set_unit(&mut self) {
        let amp = self.amplitude();
        if amp != 0. {
            self.vx /= amp;
            self.vy /= amp;
        }
    }
}

#[cfg(test)]
mod tests_vec2 {
    use super::*;

    #[test]
    fn test_vector2_simple() {
        let vec = Vector2Rep::new(1., 1.);
        println!("x: {}, y: {}, rad: {}, deg: {}, amp: {}, right: {}, down: {}", vec.vx, vec.vy, vec.angle_rad(), vec.angle_deg(), vec.amplitude(), vec.is_facing_right(), vec.is_facing_down());
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
}

pub struct Vector3Rep {
    vx: f32,
    vy: f32,
    vz: f32,
}

impl Vector3Rep {
    pub fn new(vx: f32, vy: f32, vz: f32) -> Self {
        Vector3Rep { vx, vy, vz }
    }

    pub fn angle_rad(&self) -> (f32, f32) {
        let azimuth = match (self.vx, self.vy) {
            // Zeros cases
            (vx, vy) if (vx == 0.) & (vy > 0.) => PI / 2.,
            (vx, vy) if (vx == 0.) & (vy < 0.) => 3. * PI / 2.,
            (vx, vy) if (vy == 0.) & (vx > 0.) => 0.,
            (vx, vy) if (vy == 0.) & (vx < 0.) => PI,
            // Dial quarters
            (vx, vy) if (vx > 0.) & (vy > 0.) => (vy / vx).atan(),
            (vx, vy) if (vx < 0.) & (vy > 0.) => (-vx / vy).atan() + PI / 2.,
            (vx, vy) if (vx < 0.) & (vy < 0.) => (vy / vx).atan() + PI,
            (vx, vy) if (vx > 0.) & (vy < 0.) => (-vx / vy).atan() + 3. * PI / 2.,
            // Bin, zero or vertical vector (0., 0., _)
            _ => 0.,
        };

        let dip = match (self.vx, self.vy, self.vz) {
            // Vertical cases
            (vx, vy, vz) if (vx == 0.) & (vy == 0.) & (vz > 0.) => PI / 2.,
            (vx, vy, vz) if (vx == 0.) & (vy == 0.) & (vz < 0.) => - PI / 2.,
            // General cases
            (vx, vy, vz) if (vx != 0.) & (vy != 0.) & (vz != 0.) => (vz / (vx * vx + vy * vy).sqrt()).atan(),
            // Zero vector (0., 0., 0.)
            _ => 0.,
        };

        (azimuth, dip)
    }

    pub fn angle_deg(&self) -> (f32, f32) {
        let (azimuth, dip) = self.angle_rad();
        (rad2deg(azimuth), rad2deg(dip))
    }

    pub fn from_rad(azimuth: f32, dip: f32) -> Self {
        // Generate unit vector

        // check bounds
        // ultimately need to wrap the value into the right bounds
        assert!((azimuth >= 0.) & (azimuth < 2. * PI));
        assert!((dip >= - PI / 2.) & (dip <= PI / 2.));

        let vz = dip.tan();
        let mirror_length = (1. - vz * vz).sqrt();

        let (vx, vy) = match azimuth {
            az if az == 0. => (mirror_length, 0.),
            az if az == PI / 2. => (0., mirror_length),
            az if az == PI => (- mirror_length, 0.),
            az if az == 3. * PI / 2. => (0., - mirror_length),
            az if az < PI / 2. => (az.cos(), az.sin()),
            az if az < PI => (- (az - PI / 2.).sin(), (az - PI / 2.).cos()),
            az if az < 3. * PI / 2. => (-(az - PI).cos(), -(az - PI).sin()),
            az => ((az - 3. * PI / 2.).sin(), -(az - 3. * PI / 2.).cos()),
        };

        Vector3Rep::new(vx, vy, vz)
    }

    pub fn from_deg(azimuth: f32, dip: f32) -> Self {
        let azimuth_rad = deg2rad(azimuth);
        let dip_rad = deg2rad(dip);

        Self::from_rad(azimuth_rad, dip_rad)
    }

    pub fn amplitude(&self) -> f32 {
        (self.vx * self.vx + self.vy * self.vy + self.vz * self.vz).sqrt()
    }

    pub fn set_unit(&mut self) {
        let amp = self.amplitude();
        if amp != 0. {
            self.vx /= amp;
            self.vy /= amp;
            self.vz /= amp;
        }
    }

    pub fn inner_product(&self, other: Self) -> f32 {
        self.vx * other.vx + self.vy * other.vy + self.vz * other.vz
    }

    pub fn from_vertical_section(section_vec: Vector2Rep, azimuth_rad: f32) -> Self {
        let Vector2Rep {
            vx: v2x,
            vy: v2y
        } = section_vec;

        // Vertical component is conserved
        let v3z = v2y;

        // Horizontal component is distributed between x and y
        let v3x = 0.;
        let v3y = 0.;
        todo!();

        Vector3Rep::new(v3x, v3y, v3z)
    }
}

#[cfg(test)]
mod test_vec3 {
    use super::*;

    // #[test]
    fn all() {
        let az: Vec<f32> = (0..36).map(|k| (k * 10) as f32).collect();
        let dip: Vec<f32> = (-9..=9).map(|k| (k * 10) as f32).collect();
        for a in &az {
            for d in &dip {
                let vec = Vector3Rep::from_deg(*a, *d);
                println!("az: {}, d: {}, x: {}, y: {}, z: {}", a, d, vec.vx, vec.vy, vec.vz);
            }
        }
    }

    #[test]
    fn test_vector3_simple() {
        let vec = Vector3Rep::new(1., 1., 1.);
        let (azimuth, dip) = vec.angle_rad();
        assert_approx_eq!(azimuth, PI / 4.);
        assert_approx_eq!(dip, (1.0_f32 / 2.0_f32.sqrt()).atan());
        println!("x: {}, y: {}, z: {}, rad: {}|{}, deg: {}|{}", vec.vx, vec.vy, vec.vz, vec.angle_rad().0, vec.angle_rad().1, vec.angle_deg().0, vec.angle_deg().1);
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

}

pub fn rad2deg(rad: f32) -> f32 {
    rad * 180. / PI
}

pub fn deg2rad(deg: f32) -> f32 {
    deg * PI / 180.
} 
