use crate::vectors::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }
    /// Determines if this ray points towards or away from a point.
    ///
    /// # Arguments
    ///
    /// * `point`: The point to test
    ///
    /// returns: 1 if it points away, -1 if towards, 0 if neither
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn points_away(&self, point: Vector3) -> i8 {
        let adj_pt = point - self.origin;
        let dt = adj_pt.dot(&self.direction);
        if dt == 0.0 {
            0
        } else if dt > 0.0 {
            -1
        } else {
            1
        }
    }
}
