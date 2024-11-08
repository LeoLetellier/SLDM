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

        let x_sign = if is_facing_right {1.0} else {-1.0};
        let (vx, vy) = match slope {
            s if s == PI / 2. => (0., 1.),
            s if s == - PI / 2. => (0., -1.),
            s => (x_sign * s.cos().abs(), s.sin()),
        };

        Vector2Rep::new(vx, vy)
    }

    pub fn from_deg(slope: f32, is_facing_right: bool) -> Self {
        let slope_rad = deg2rad(slope);
        Vector2Rep::from_rad(slope_rad, is_facing_right)
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

    #[test]
    fn test_from_angle_right() {
        let vec = Vector2Rep::from_deg(0.0, true);
        assert_eq!((vec.vx, vec.vy), (1.0, 0.0));
    }

    #[test]
    fn test_from_angle_left() {
        let vec = Vector2Rep::from_deg(0.0, false);
        assert_eq!((vec.vx, vec.vy), (-1.0, 0.0));
    }

    #[test]
    fn test_from_angle_up() {
        let vec = Vector2Rep::from_deg(90., true);
        assert_eq!((vec.vx, vec.vy), (0.0, 1.0));
    }

    #[test]
    fn test_from_angle_down() {
        let vec = Vector2Rep::from_deg(-90., true);
        assert_eq!((vec.vx, vec.vy), (0.0, -1.0));
    }

    #[test]
    fn test_from_angle_diag() {
        let vec = Vector2Rep::from_deg(-45., false);
        assert_eq!((vec.vx, vec.vy), (-1. / (2.0_f32.sqrt()), -1. / (2.0_f32.sqrt())));
    }

    // #[test]
    fn from_all_angle() {
        let slope: Vec<f32> = (-9..=9).map(|k| (k * 10) as f32).collect();
        for s in &slope {
            let vec = Vector2Rep::from_deg(*s, true);
            println!("slope {}/{}, x: {}, y: {}", s, true, vec.vx, vec.vy);
        }
        for s in &slope {
            let vec = Vector2Rep::from_deg(*s, false);
            println!("slope {}/{}, x: {}, y: {}", s, false, vec.vx, vec.vy);
        }
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

    // pub fn angle_rad(&self) -> (f32, f32) {
    //     let amp = self.amplitude();
    //     let dip = (self.vz / amp).acos();
    //     let azimuth = (self.vy / self.vx).atan();

    //     (azimuth, dip)
    // }

    pub fn angle_rad(&self) -> (f32, f32) {
        let mut azimuth = - (self.vx / self.vy).atan();
        match (self.vx, self.vy) {
            (_vx, vy) if (vy < 0.) => azimuth += 3. * PI / 2.,
            (_vx, vy) if (vy > 0.) => azimuth += PI / 2.,
            (vx, vy) if (vy == 0.) & (vx < 0.) => azimuth = PI,
            _ => azimuth = 0.,
        };

        let dip = self.vz.asin();

        (azimuth, dip)
    }

    pub fn angle_deg(&self) -> (f32, f32) {
        let (azimuth, dip) = self.angle_rad();
        (rad2deg(azimuth), rad2deg(dip))
    }

    pub fn from_rad(azimuth: f32, dip: f32) -> Self {
        let mut vx = azimuth.cos() * dip.cos();
        let mut vy = azimuth.sin() * dip.cos();
        let vz = dip.sin();

        if (dip == PI / 2.) | (dip == - PI / 2.) {
            (vx, vy) = (0., 0.);
        }

        Vector3Rep { vx, vy, vz }
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

    #[test]
    fn all() {
        let az: Vec<f32> = (0..36).map(|k| (k * 10) as f32).collect();
        let dip: Vec<f32> = (-9..=9).map(|k| (k * 10) as f32).collect();
        for a in &az {
            for d in &dip {
                let vec = Vector3Rep::from_deg(*a, *d);
                // assert_approx_eq!(vec.amplitude(), 1.0);
                println!("az: {}, d: {}, x: {}, y: {}, z: {}, amp: {}", a, d, vec.vx, vec.vy, vec.vz, vec.amplitude());
                println!("az: {}, d: {}", vec.angle_deg().0, vec.angle_deg().1);
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
