use crate::ray::Ray;
use crate::vectors::{Vector2, Vector3};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SimpleTriangle {
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl SimpleTriangle {
    pub fn new(a: Vector3, b: Vector3, c: Vector3) -> SimpleTriangle {
        SimpleTriangle { a, b, c }
    }

    /// Computes the normal of the triangle. Normal is defined as ABxAC, or a vector with the length of the area of the triangle.
    ///
    /// returns: Result<Vector3, String>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    ///     //TODO:test
    pub fn normal(&self) -> Vector3 {
        (self.b - self.a).cross(&(self.c - self.b)) / 2.0
    }
    pub fn center(&self) -> Vector3 {
        (self.a + self.b + self.c) / 3.0
    }
    pub fn normal_ray(&self) -> Ray {
        Ray::new(self.center(), self.normal())
    }
    pub fn area(&self) -> f64 {
        self.normal().magnitude()
    }
    //TODO:test
    pub fn point_intersects(&self, other: Vector3) -> bool {
        let other_adj = other - self.a;
        let b_adj = self.b - self.a;
        let c_adj = self.c - self.a;
        if other_adj.dot(&self.normal()) != 0.0 {
            false
        } else {
            let coord = Vector2::new(
                other_adj.dot(&b_adj) / (b_adj.dot(&b_adj)),
                other_adj.dot(&c_adj) / (c_adj.dot(&c_adj)),
            );
            (coord.x >= 0.0) && (coord.y >= 0.0) && (coord.x + coord.y <= 1.0)
        }
    }
    //TODO:test
    pub fn ray_intersects(&self, other: Ray) -> Result<Vector3, String> {
        let origin = self.a;
        let x = self.b - self.a;
        let y = self.c - self.a;
        let adjusted_ray = Ray::new(other.origin - origin, other.direction);

        if adjusted_ray.direction.dot(&self.normal()) == 0.0
            && adjusted_ray.origin.dot(&self.normal()) != 0.0
        {
            Err("Ray is perpendicular to plane".to_string())
        } else if adjusted_ray.direction.dot(&self.normal()) == 0.0 {
            Err("Ray lies completely on plane".to_string())
        } else {
            let no = self.normal().dot(&adjusted_ray.origin);
            let nv = self.normal().dot(&adjusted_ray.direction);
            if -no / nv < 0.0 {
                Err("Ray points away from plane".to_string())
            } else {
                let pt = adjusted_ray.origin + adjusted_ray.direction * (-no / nv) + origin;
                if self.point_intersects(pt) {
                    Ok(pt)
                } else {
                    Err("Ray intersects outside of triangle".to_string())
                }
            }
        }
    }
}
