use crate::vectors::Vector2;

#[derive(Debug, PartialEq, Copy, Clone)]
struct Triangle {
    a: Vector2,
    b: Vector2,
    c: Vector2,
}

impl Triangle {
    pub fn new(a: Vector2, b: Vector2, c: Vector2) -> Self {
        Self { a, b, c }
    }
    pub fn area(&self) -> f64 {
        (self.b - self.a).cross(&(self.c - self.b)) / 2.0
    }
    pub fn point_intersects(&self, pt: Vector2) -> bool {
        (pt.x >= 0.0) && (pt.y >= 0.0) && (pt.x + pt.y <= 1.0)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Polygon {
    points: Vec<Vector2>,
}

impl Polygon {
    // TODO: add a second constructor for dxf-style inputs (points, order of pts)
    pub fn new(points: Vec<Vector2>) -> Self {
        Self { points } // TODO: add safeguards to make sure it is CCW
    }
    pub fn area(&self) -> f64 {
        let mut sum = 0.0;
        for i in 1..self.points.len() - 1 {
            sum += Triangle::new(
                self.points[0].clone(),
                self.points[i].clone(),
                self.points[i + 1].clone(),
            )
            .area();
        }
        sum
    }
}
