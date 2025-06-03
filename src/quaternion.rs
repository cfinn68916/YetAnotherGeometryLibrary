use crate::vectors::Vector3;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Default for Quaternion {
    fn default() -> Quaternion {
        Quaternion {
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

impl ops::Add<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn add(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            w: self.w + rhs.w,
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Sub<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn sub(self, rhs: Quaternion) -> Self::Output {
        Quaternion {
            w: self.w - rhs.w,
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Div<f64> for Quaternion {
    type Output = Quaternion;
    fn div(self, rhs: f64) -> Self::Output {
        Quaternion {
            w: self.w / rhs,
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::Mul<f64> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: f64) -> Self::Output {
        Quaternion {
            w: self.w * rhs,
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;
    fn mul(self, rhs: Quaternion) -> Self::Output {
        let v = rhs.get_vector() * self.get_scalar()
            + self.get_vector() * rhs.get_scalar()
            + self.get_vector().cross(&rhs.get_vector());
        Quaternion {
            w: self.get_scalar() * rhs.get_scalar() - self.get_vector().dot(&rhs.get_vector()),
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Self {
        Self { w, x, y, z }
    }
    pub fn from_rotation_vector(rvec: Vector3) -> Self {
        let theta = rvec.magnitude();
        if theta.abs() < 1E-9 {
            Quaternion::default()
        } else {
            Self::from_scalar_vector((theta / 2.0).cos(), rvec.hat() * (theta / 2.0).sin())
        }
    }
    pub(crate) fn from_scalar_vector(scalar: f64, vector: Vector3) -> Self {
        Quaternion::new(scalar, vector.x, vector.y, vector.z)
    }
    fn get_scalar(&self) -> f64 {
        self.w
    }
    pub(crate) fn get_vector(&self) -> Vector3 {
        Vector3::new(self.x, self.y, self.z)
    }
    pub fn dot(&self, rhs: &Quaternion) -> f64 {
        self.w * rhs.w + self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn conjugate(&self) -> Self {
        Quaternion::new(self.w, -self.x, -self.y, -self.z)
    }
    pub fn inverse(&self) -> Self {
        self.conjugate() / self.dot(self)
    }
    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }
    pub fn hat(&self) -> Self {
        let norm = self.norm();
        if norm == 0.0 {
            Quaternion::default()
        } else {
            Quaternion::new(self.w / norm, self.x / norm, self.y / norm, self.z / norm)
        }
    }
    pub fn exp(&self) -> Self {
        let scalar_exp = self.get_scalar().exp();
        let axial_scalar = if self.get_vector().magnitude() < 1E-9 {
            let axial_mag2 = self.get_vector().magnitude().powi(2);
            let axial_mag4 = axial_mag2 * axial_mag2;
            1.0 - (axial_mag2 / 6.0) + (axial_mag4 / 120.0)
        } else {
            let axial_mag = self.get_vector().magnitude();
            axial_mag.sin() / axial_mag
        };
        Quaternion::from_scalar_vector(
            self.get_vector().magnitude().cos() * scalar_exp,
            self.get_vector() * axial_scalar * scalar_exp,
        )
    }
}
