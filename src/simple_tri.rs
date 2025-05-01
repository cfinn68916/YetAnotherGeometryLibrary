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
        (self.b - self.a).cross(&(self.c - self.b))/2.0
    }
    pub fn area(&self) -> f64{
        self.normal().magnitude()
    }
    pub fn point_intersects(&self,other:Vector3) -> bool {
        let other_adj=other-self.a;
        let b_adj=self.b-self.a;
        let c_adj=self.c-self.a;
        if other_adj.dot(&self.normal())!=0.0 {
            false
        }else{
            let coord=Vector2::new(other_adj.dot(&b_adj)/(b_adj.dot(&b_adj)),other_adj.dot(&c_adj)/(c_adj.dot(&c_adj)));
            (coord.x>=0.0)&&(coord.y>=0.0)&&(coord.x+coord.y<=1.0)
        }
    }
}
