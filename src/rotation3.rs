use crate::quaternion::Quaternion;
use crate::vectors::Vector3;
use std::ops;

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
//
// impl ops::Mul<f64> for Rotation3 {
//     type Output = Rotation3;
//     fn mul(self, other: f64) -> Self::Output {}
// }

impl Rotation3 {
    pub fn identity() -> Rotation3 {
        Rotation3 {
            q: Quaternion::default(),
        }
    }
    pub fn new(q: Quaternion) -> Rotation3 {
        Rotation3 { q }
    }
    pub fn from_axis_angle(v: Vector3) -> Rotation3 {
        Rotation3 {
            q: Quaternion::from_rotation_vector(v),
        }
    }
}
