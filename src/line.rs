use crate::vectors::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Line {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Line {
    pub fn new(origin: Vector3, direction2: Vector3) -> Line {
        let direction = direction2.normd();
        Line { origin, direction }
    }
    pub fn at(&self, t: f64) -> Vector3 {
        self.origin + (self.direction * t)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct LineSegment {
    pub a: Vector3,
    pub b: Vector3,
}
impl LineSegment {
    pub fn new(a: Vector3, b: Vector3) -> LineSegment {
        LineSegment { a, b }
    }
}
