use crate::vectors::Vector3;
use core::ops;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix3 {
    pub data: [[f64; 3]; 3],
}
impl ops::Neg for Matrix3 {
    type Output = Matrix3;
    fn neg(self) -> Self::Output {
        Self::new([
            [-self.data[0][0], -self.data[0][1], -self.data[0][2]],
            [-self.data[1][0], -self.data[1][1], -self.data[1][2]],
            [-self.data[2][0], -self.data[2][1], -self.data[2][2]],
        ])
    }
}
impl ops::Mul<Vector3> for Matrix3 {
    type Output = Vector3;
    fn mul(self, rhs: Vector3) -> Self::Output {
        let row_vecs = self.row_vectors();
        Vector3::new(
            rhs.dot(&row_vecs[0]),
            rhs.dot(&row_vecs[1]),
            rhs.dot(&row_vecs[2]),
        )
    }
}
impl ops::Mul<f64> for Matrix3 {
    type Output = Matrix3;
    fn mul(self, rhs: f64) -> Self::Output {
        Matrix3::new([
            [
                self.data[0][0] * rhs,
                self.data[0][1] * rhs,
                self.data[0][2] * rhs,
            ],
            [
                self.data[1][0] * rhs,
                self.data[1][1] * rhs,
                self.data[1][2] * rhs,
            ],
            [
                self.data[2][0] * rhs,
                self.data[2][1] * rhs,
                self.data[2][2] * rhs,
            ],
        ])
    }
}

impl Matrix3 {
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        Self { data }
    }
    pub fn identity() -> Self {
        Self::new([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }
    fn from_flat(data: [f64; 9]) -> Self {
        Self::new([
            [data[0], data[1], data[2]],
            [data[3], data[4], data[5]],
            [data[6], data[7], data[8]],
        ])
    }
    pub fn row_vectors(&self) -> Vec<Vector3> {
        vec![
            Vector3::new(self.data[0][0], self.data[0][1], self.data[0][2]),
            Vector3::new(self.data[1][0], self.data[1][1], self.data[1][2]),
            Vector3::new(self.data[2][0], self.data[2][1], self.data[2][2]),
        ]
    }
    fn flat_array(&self) -> [f64; 9] {
        [
            self.data[0][0],
            self.data[0][1],
            self.data[0][2],
            self.data[1][0],
            self.data[1][1],
            self.data[1][2],
            self.data[2][0],
            self.data[2][1],
            self.data[2][2],
        ]
    }
    pub fn determinant(&self) -> f64 {
        self.data[0][0] * (self.data[1][1] * self.data[2][2] - self.data[1][2] * self.data[2][1])
            - self.data[0][1]
                * (self.data[1][0] * self.data[2][2] - self.data[1][2] * self.data[2][0])
            + self.data[0][2]
                * (self.data[1][0] * self.data[2][1] - self.data[1][1] * self.data[2][0])
    }
    pub fn inverse(&self) -> Option<Self> {
        let mat = self.flat_array();
        let det = self.determinant();
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
            Some(Self::from_flat(adj) * (1.0 / det))
        }
    }
}
