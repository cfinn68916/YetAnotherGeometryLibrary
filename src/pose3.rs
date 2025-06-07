use crate::rotation3::Rotation3;
use crate::vectors::Vector3;
use std::ops;

/// A pose (position plus orientation) in 3d
///
/// # Examples
///
/// ```
/// use YetAnotherGeometryLibrary::pose3::Pose3;
/// use YetAnotherGeometryLibrary::rotation3::Rotation3;
/// use YetAnotherGeometryLibrary::vectors::Vector3;
/// let fwd1r90=Pose3::new(Vector3::i_hat(),Rotation3::from_axis_angle(Vector3::k_hat()*std::f64::consts::FRAC_PI_2));
/// let r90=Pose3::new(Vector3::zero(),Rotation3::from_axis_angle(Vector3::k_hat()*std::f64::consts::FRAC_PI_2));
/// let id=Pose3::identity();
/// // A+B is A then B
/// assert_eq!(fwd1r90+r90, Pose3::new(Vector3::i_hat(),Rotation3::from_axis_angle(Vector3::k_hat()*std::f64::consts::PI)));
/// assert_eq!(r90+fwd1r90, Pose3::new(Vector3::j_hat(),Rotation3::from_axis_angle(Vector3::k_hat()*std::f64::consts::PI)));
/// // Unary minus represents inverse
/// assert_eq!(fwd1r90+(-fwd1r90),id);
/// // Right inverse and Left inverse must be the same
/// assert_eq!((-fwd1r90)+fwd1r90,id);
/// // It is not commutative
/// assert_ne!(r90+fwd1r90,fwd1r90+r90);
/// // Multiplication by a scalar scales the translation and rotation
/// assert_eq!(fwd1r90*1.5, Pose3::new(Vector3::i_hat()*1.5,Rotation3::from_axis_angle(Vector3::k_hat()*std::f64::consts::PI*0.75)));
/// // It is not equivalent to addition
/// assert_ne!(fwd1r90+fwd1r90,fwd1r90*2.0);
/// ```
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Pose3 {
    pub position: Vector3,
    pub orientation: Rotation3,
}
impl ops::Add<Pose3> for Pose3 {
    type Output = Pose3;
    fn add(self, rhs: Pose3) -> Pose3 {
        Self::new(
            self.position + self.orientation.rotate_vector(rhs.position),
            self.orientation + rhs.orientation,
        )
    }
}
impl ops::Neg for Pose3 {
    type Output = Pose3;
    fn neg(self) -> Pose3 {
        Pose3::new(
            (-self.orientation).rotate_vector(-self.position),
            -self.orientation,
        )
    }
}
impl ops::Sub<Pose3> for Pose3 {
    type Output = Pose3;
    fn sub(self, rhs: Pose3) -> Pose3 {
        Self::new(
            (-rhs.orientation).rotate_vector(self.position - rhs.position),
            self.orientation - rhs.orientation,
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
    pub fn identity() -> Self {
        Self::new(Vector3::zero(), Rotation3::identity())
    }
}
