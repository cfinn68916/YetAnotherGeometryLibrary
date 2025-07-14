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
    /// use YetAnotherGeometryLibrary::ray;
    /// use YetAnotherGeometryLibrary::ray::Ray;
    /// use YetAnotherGeometryLibrary::vectors::Vector3;
    ///
    /// let pt=Vector3::new(2.0,1.0,5.0);
    /// assert_eq!(Ray::new(Vector3::new(2.0,1.0,5.0),Vector3::new(1.0,0.0,0.0)).points_away(pt),0);
    /// assert_eq!(Ray::new(Vector3::new(3.0,2.0,6.0),Vector3::new(1.0,0.0,0.0)).points_away(pt),1);
    /// assert_eq!(Ray::new(Vector3::new(3.0,2.0,6.0),Vector3::new(-1.0,0.0,0.0)).points_away(pt),-1);
    /// assert_eq!(Ray::new(Vector3::new(3.0,2.0,6.0),Vector3::new(-1.0,1.0,0.0)).points_away(pt),0);
    /// ```
    pub fn points_away(&self, point: Vector3) -> i8 {
        let adj_pt = point - self.origin;
        let dt = adj_pt.dot(&self.direction);
        if dt == 0.0 {
            0
        } else if dt > 0.0 {
            return -1;
        } else {
            return 1;
        }
    }
}
