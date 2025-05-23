use std::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl ops::Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl ops::Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, rhs: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl ops::Div<f64> for Vector3 {
    type Output = Vector3;
    fn div(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl ops::Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 { x, y, z }
    }
    pub fn ihat() -> Vector3 {
        Vector3::new(1.0, 0.0, 0.0)
    }
    pub fn jhat() -> Vector3 {
        Vector3::new(0.0, 1.0, 0.0)
    }
    pub fn khat() -> Vector3 {
        Vector3::new(0.0, 0.0, 1.0)
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0, 0.0, 0.0)
    }
    pub(crate) fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn dot(&self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
    pub fn angle_cosine(&self, rhs: &Vector3) -> f64 {
        self.dot(rhs) / (self.magnitude() * rhs.magnitude())
    }
    pub fn normd(&self) -> Vector3 {
        if self.x == 0.0 && self.y == 0.0 {
            *self
        } else {
            (*self) / self.magnitude()
        }
    }
    pub fn with_magnitude(&self, magnitude: f64) -> Vector3 {
        self.normd() * magnitude
    }
}

impl ops::Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl ops::Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, rhs: Vector2) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl ops::Mul<f64> for Vector2 {
    type Output = Vector2;
    fn mul(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl ops::Div<f64> for Vector2 {
    type Output = Vector2;
    fn div(self, rhs: f64) -> Self::Output {
        Vector2 {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}
impl ops::Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Self::Output {
        Vector2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Vector2 {
        Vector2 { x, y }
    }
    pub fn zero() -> Vector2 {
        Vector2::new(0.0, 0.0)
    }
    fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dot(&self, rhs: &Vector2) -> f64 {
        self.x * rhs.x + self.y * rhs.y
    }
    pub fn cross(&self, rhs: &Vector2) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }
    pub fn angle_cosine(&self, rhs: &Vector2) -> f64 {
        self.dot(rhs) / (self.magnitude() * rhs.magnitude())
    }
    pub fn normd(&self) -> Vector2 {
        if self.x == 0.0 && self.y == 0.0 {
            *self
        } else {
            (*self) / self.magnitude()
        }
    }
}
