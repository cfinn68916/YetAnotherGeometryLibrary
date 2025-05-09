use crate::line::{Line, LineSegment};
use crate::ray::Ray;
use crate::vectors::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct SimplePlane {
    pub origin: Vector3,
    pub normal: Vector3,
}

fn inv33(mat: [f64; 9]) -> Option<[f64; 9]> {
    let det = mat[0] * (mat[4] * mat[8] - mat[5] * mat[7])
        - mat[1] * (mat[3] * mat[8] - mat[5] * mat[6])
        + mat[2] * (mat[3] * mat[7] - mat[4] * mat[6]);
    if det == 0.0 {
        None
    } else {
        let cof = [
            mat[4] * mat[8] - mat[5] * mat[7],
            mat[5] * mat[6] - mat[3] * mat[8],
            mat[3] * mat[7] - mat[4] * mat[6],
            mat[2] * mat[7] - mat[1] * mat[6],
            mat[0] * mat[8] - mat[2] * mat[6],
            mat[1] * mat[6] - mat[0] * mat[7],
            mat[1] * mat[5] - mat[2] * mat[4],
            mat[2] * mat[3] - mat[0] * mat[5],
            mat[0] * mat[4] - mat[1] * mat[3],
        ];
        let adj = [
            cof[0], cof[3], cof[6], cof[1], cof[4], cof[7], cof[2], cof[5], cof[8],
        ];
        Some(adj.map(|v| v / det))
    }
}
fn matmulvec(mat: [f64; 9], v: [f64; 3]) -> [f64; 3] {
    [
        mat[0] * v[0] + mat[1] * v[1] + mat[2] * v[2],
        mat[3] * v[0] + mat[4] * v[1] + mat[5] * v[2],
        mat[6] * v[0] + mat[7] * v[1] + mat[8] * v[2],
    ]
}

impl SimplePlane {
    pub fn new(origin: Vector3, normal2: Vector3) -> Self {
        let normal = normal2.normd();
        Self { origin, normal }
    }
    pub fn from_mxb(mx: f64, my: f64, c: f64) -> Self {
        SimplePlane::new(Vector3::new(0.0, 0.0, c), Vector3::new(mx, my, -1.0))
    }
    pub fn regress(points: &Vec<Vector3>) -> (SimplePlane, f64) {
        let mut sum_x = 0.0;
        let mut sum_y = 0.0;
        let mut sum_z = 0.0;
        let mut sum_xy = 0.0;
        let mut sum_xz = 0.0;
        let mut sum_yz = 0.0;
        let mut sum_x2 = 0.0;
        let mut sum_y2 = 0.0;
        for p in points {
            sum_x += p.x;
            sum_y += p.y;
            sum_z += p.z;
            sum_x2 += p.x * p.x;
            sum_y2 += p.y * p.y;
            sum_xy += p.x * p.y;
            sum_xz += p.x * p.z;
            sum_yz += p.y * p.z;
        }
        let forward_matrix = [
            sum_x2, sum_xy, sum_x, sum_xy, sum_y2, sum_y, sum_x, sum_y, 1.0,
        ];
        let reverse_matrix = inv33(forward_matrix).unwrap();
        let v = [sum_xz, sum_yz, sum_z];
        let param_vector = matmulvec(reverse_matrix, v);
        let mx = param_vector[0];
        let my = param_vector[1];
        let c = param_vector[2];
        let mut square_error = 0.0;
        for pt in points {
            square_error += (pt.x * mx + pt.y * my + c - pt.z).powi(2);
        }
        (Self::from_mxb(mx, my, c), square_error)
    }

    //TODO:test
    pub fn point_intersects(&self, other: Vector3) -> bool {
        self.normal.dot(&(self.origin - other)) == 0.0
    }
    /// Determines if/where a ray will hit this plane. If the ray lies on the plane, it returns the origin of the ray. If it hits once (ie: origin is on the plane) it returns the origin.
    ///
    /// # Arguments
    ///
    /// * `other`: The ray to test
    ///
    /// returns: Result<Vector3, String>
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn ray_intersects(&self, other: Ray) -> Result<Vector3, String> {
        let adjusted_ray = Ray::new(other.origin - self.origin, other.direction);
        if adjusted_ray.direction.dot(&self.normal) == 0.0
            && adjusted_ray.origin.dot(&self.normal) != 0.0
        {
            Err("Ray is perpendicular to plane".to_string())
        } else if adjusted_ray.direction.dot(&self.normal) == 0.0 {
            Ok(other.origin)
        } else {
            let no = self.normal.dot(&adjusted_ray.origin);
            let nv = self.normal.dot(&adjusted_ray.direction);
            if -no / nv < 0.0 {
                Err("Ray points away from plane".to_string())
            } else {
                Ok(adjusted_ray.origin + adjusted_ray.direction * (-no / nv) + self.origin)
            }
        }
    }
    //TODO:test
    pub fn line_intersects(&self, other: Line) -> Result<Vector3, String> {
        let adjusted_line = Line::new(other.origin - self.origin, other.direction);
        if adjusted_line.direction.dot(&self.normal) == 0.0
            && adjusted_line.origin.dot(&self.normal) != 0.0
        {
            Err("Line is perpendicular to plane".to_string())
        } else if adjusted_line.direction.dot(&self.normal) == 0.0 {
            Ok(other.origin)
        } else {
            let no = self.normal.dot(&adjusted_line.origin);
            let nv = self.normal.dot(&adjusted_line.direction);
            Ok(adjusted_line.origin + adjusted_line.direction * (-no / nv) + self.origin)
        }
    }
    //TODO:test
    pub fn segment_intersects(&self, other: LineSegment) -> Result<Vector3, String> {
        let adjusted_line = LineSegment::new(other.a - self.origin, other.b - self.origin);
        if (adjusted_line.b - adjusted_line.a).dot(&self.normal) == 0.0
            && adjusted_line.a.dot(&self.normal) != 0.0
        {
            Err("Line is perpendicular to plane".to_string())
        } else if (adjusted_line.b - adjusted_line.a).dot(&self.normal) == 0.0 {
            Ok(other.a)
        } else {
            let no = self.normal.dot(&adjusted_line.a);
            let nv = self.normal.dot(&(adjusted_line.b - adjusted_line.a));
            if 1.0 >= (-no / nv) && (-no / nv) >= 0.0 {
                Ok(adjusted_line.b * (-no / nv) + self.origin)
            } else {
                Err("Segment is outside the plane".to_string())
            }
        }
    }
}

pub struct CoordinatePlane {
    pub origin: Vector3,
    pub x: Vector3,
    pub y: Vector3,
}

impl CoordinatePlane {
    pub fn new(origin: Vector3, x: Vector3, y: Vector3) -> Result<Self, String> {
        if x.dot(&y) != 0.0 {
            Err("Non-orthagnal vectors cannot form a plane".to_string())
        } else {
            Ok(Self { origin, x, y })
        }
    }
}
