use crate::rotation3::Rotation3;
use crate::vectors::Vector3;
use std::ops;

pub struct Pose3 {
    position: Vector3,
    orientation: Rotation3,
}
impl ops::Add<Pose3> for Pose3 {
    type Output = Pose3;
    fn add(self, rhs: Pose3) -> Pose3 {
        Self::new(
            self.position + self.orientation.rotate_vector(rhs.position),
            rhs.orientation + self.orientation,
        )
    }
}
impl ops::Neg for Pose3 {
    type Output = Pose3;
    fn neg(self) -> Pose3 {
        Pose3::new(
            (-self.orientation).rotate_vector(-self.position),
            -self.orientation.clone(),
        )
    }
}
impl ops::Sub<Pose3> for Pose3 {
    type Output = Pose3;
    fn sub(self, rhs: Pose3) -> Pose3 {
        Self::new(
            self.position + self.orientation.rotate_vector(rhs.position),
            rhs.orientation + self.orientation,
        )
    }
}
impl ops::Mul<f64> for Pose3 {
    type Output = Pose3;
    fn mul(self, rhs: f64) -> Pose3 {
        Self::new(self.position * rhs, self.orientation * rhs)
    }
}

impl Pose3 {
    pub fn new(position: Vector3, orientation: Rotation3) -> Self {
        Self {
            position,
            orientation,
        }
    }
    pub fn zero() -> Self {
        Self::new(Vector3::zero(), Rotation3::identity())
    }
}
