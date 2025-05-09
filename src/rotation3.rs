use crate::quaternion::Quaternion;
use crate::vectors::Vector3;
use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Rotation3 {
    pub q: Quaternion,
}

impl ops::Neg for Rotation3 {
    type Output = Rotation3;
    fn neg(self) -> Self::Output {
        Rotation3 {
            q: self.q.inverse(),
        }
    }
}

impl ops::Add<Rotation3> for Rotation3 {
    type Output = Rotation3;
    fn add(self, other: Rotation3) -> Self::Output {
        Rotation3 {
            q: other.q * self.q,
        }
    }
}

impl ops::Sub<Rotation3> for Rotation3 {
    type Output = Rotation3;
    fn sub(self, other: Rotation3) -> Self::Output {
        Rotation3 {
            q: other.q.inverse() * self.q,
        }
    }
}

impl ops::Mul<f64> for Rotation3 {
    type Output = Rotation3;
    fn mul(self, other: f64) -> Self::Output {
        if self.q.w >= 0.0 {
            Rotation3::from_axis_angle(
                self.q
                    .get_vector()
                    .with_magnitude(2.0 * other * self.q.w.acos()),
            )
        } else {
            Rotation3::from_axis_angle(
                -self
                    .q
                    .get_vector()
                    .with_magnitude(2.0 * other * (-self.q.w).acos()),
            )
        }
    }
}
impl ops::Div<f64> for Rotation3 {
    type Output = Rotation3;
    fn div(self, other: f64) -> Self::Output {
        self * (1.0 / other)
    }
}

impl Rotation3 {
    pub fn identity() -> Rotation3 {
        Rotation3 {
            q: Quaternion::default(),
        }
    }
    pub fn new(q: Quaternion) -> Rotation3 {
        Rotation3 { q }
    }
    pub fn rotate_vector(&self, vector: Vector3) -> Vector3 {
        // https://en.wikipedia.org/wiki/Quaternions_and_spatial_rotation#Using_quaternions_as_rotations
        // Accessed 2025/5/9
        let other_as_quaternion = Quaternion::from_scalar_vector(0.0, vector);
        (other_as_quaternion.clone() * self.q * (other_as_quaternion.inverse())).get_vector()
    }
    pub fn from_axis_angle(v: Vector3) -> Rotation3 {
        Rotation3 {
            q: Quaternion::from_rotation_vector(v),
        }
    }
}
