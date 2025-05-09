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
        let adj_pt = pt - self.a;
        let x = (self.b - self.a) / (self.b - self.a).dot(&(self.b - self.a));
        let y = (self.c - self.a) / (self.c - self.a).dot(&(self.c - self.a));
        let lcs_pt = Vector2::new(x.dot(&adj_pt), y.dot(&adj_pt));
        (lcs_pt.x >= 0.0) && (lcs_pt.y >= 0.0) && (lcs_pt.x + lcs_pt.y <= 1.0)
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
