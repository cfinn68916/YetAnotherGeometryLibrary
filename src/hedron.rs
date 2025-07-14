use crate::simple_tri::SimpleTriangle;
use crate::vectors::Vector3;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tetrahedron {
    pub origin: Vector3,
    pub a: Vector3,
    pub b: Vector3,
    pub c: Vector3,
}

impl Tetrahedron {
    pub fn new(origin: Vector3, a: Vector3, b: Vector3, c: Vector3) -> Self {
        Tetrahedron { origin, a, b, c }
    }
    pub fn from_points(a: Vector3, b: Vector3, c: Vector3, d: Vector3) -> Self {
        Tetrahedron::new(a, b - a, c - a, d - a)
    }
    pub fn volume(&self) -> f64 {
        // This is the scalar triple product, or the determinant of the matrix with a,b,and c as its rows. It is the volume of the parallelepiped with sides a,b,c.
        self.a.dot(&self.b.cross(&self.c)) / 6.0
    }
    pub fn pt_1(&self) -> Vector3 {
        self.origin
    }
    pub fn pt_2(&self) -> Vector3 {
        self.origin + self.a
    }
    pub fn pt_3(&self) -> Vector3 {
        self.origin + self.b
    }
    pub fn pt_4(&self) -> Vector3 {
        self.origin + self.c
    }
    pub fn point_in(&self, other: Vector3) -> bool {
        let other_adj = other - self.origin;
        let point_in_threespace = Vector3::new(
            other_adj.dot(&self.a) / self.a.dot(&self.a),
            other_adj.dot(&self.b) / self.b.dot(&self.b),
            other_adj.dot(&self.c) / self.c.dot(&self.c),
        );
        (point_in_threespace.x >= 0.0)
            && (point_in_threespace.y >= 0.0)
            && (point_in_threespace.z >= 0.0)
            && (point_in_threespace.dot(&Vector3::new(1.0, 1.0, 1.0)) <= 1.0)
    }
    /// Returns the surface triangles. If the volume is positive, the normals of the triangles will point outward, and if it is negative, they will point inward
    ///
    /// returns: [SimpleTriangle;4]
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// ```
    pub fn surface(&self) -> [SimpleTriangle; 4] {
        [
            SimpleTriangle::new(self.pt_1(), self.pt_3(), self.pt_2()),
            SimpleTriangle::new(self.pt_1(), self.pt_4(), self.pt_3()),
            SimpleTriangle::new(self.pt_1(), self.pt_2(), self.pt_4()),
            SimpleTriangle::new(self.pt_2(), self.pt_3(), self.pt_4()),
        ]
    }
    pub fn surface_area(&self) -> f64 {
        let sf = self.surface();
        sf[0].area() + sf[1].area() + sf[2].area() + sf[3].area()
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Polyhedron {
    points: Vec<Vector3>,
    faces: Vec<(usize, usize, usize)>,
}

impl Polyhedron {
    //TODO:Impl
    fn check_homeomorphism(points: &Vec<Vector3>, faces: &Vec<(usize, usize, usize)>) -> bool {
        true
    }
    //TODO:Impl
    fn check_normals(points: &Vec<Vector3>, faces: &Vec<(usize, usize, usize)>) -> bool {
        true
    }
    //TODO:Impl
    pub fn new(points: Vec<Vector3>, faces: Vec<(usize, usize, usize)>) -> Result<Self, String> {
        Err("NOIMPL".to_owned())
    }
    //TODO:Impl
    pub fn new_autofix(
        points: Vec<Vector3>,
        faces: Vec<(usize, usize, usize)>,
    ) -> Result<Self, String> {
        Err("NOIMPL".to_owned())
    }
    pub fn new_unchecked(points: Vec<Vector3>, faces: Vec<(usize, usize, usize)>) -> Self {
        Self { points, faces }
    }
    pub fn cube() -> Self {
        Self::new_unchecked(
            vec![
                Vector3::zero(),
                Vector3::i_hat(),
                Vector3::j_hat(),
                Vector3::k_hat(),
                Vector3::new(0.0, 1.0, 1.0),
                Vector3::new(1.0, 0.0, 1.0),
                Vector3::new(1.0, 1.0, 0.0),
                Vector3::new(1.0, 1.0, 1.0),
            ],
            vec![
                (0, 2, 1),
                (0, 1, 3),
                (0, 3, 2),
                (6, 1, 2),
                (5, 3, 1),
                (4, 2, 3),
                (4, 5, 7),
                (4, 7, 6),
                (5, 6, 7),
                (4, 3, 5),
                (4, 6, 2),
                (5, 1, 6),
            ],
        )
    }
    pub fn get_faces(&self) -> Vec<SimpleTriangle> {
        self.faces
            .iter()
            .map(|x| SimpleTriangle::new(self.points[x.0], self.points[x.1], self.points[x.2]))
            .collect()
    }
    pub fn get_volume(&self) -> f64 {
        self.faces
            .iter()
            .map(|x| {
                Tetrahedron::from_points(
                    self.points[0],
                    self.points[x.0],
                    self.points[x.1],
                    self.points[x.2],
                )
                .volume()
            })
            .sum()
    }
    pub fn get_obj(&self) -> String {
        let mut ret = "# Automatically generated from polyhederon by YAGL\n".to_string();
        for pt in &self.points {
            ret += format!("v {} {} {}\n", pt.x, pt.y, pt.z).as_str();
        }
        for f in &self.faces {
            ret += format!("f {} {} {}\n", f.0 + 1, f.1 + 1, f.2 + 1).as_str();
        }
        ret
    }
}
