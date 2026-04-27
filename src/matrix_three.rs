use super::matrix::{Matrix, Orthogonal, SquareMatrix};
use super::vec_three::Vec3;
use std::ops::{Div, Mul};

#[derive(Debug, PartialEq)]
pub struct Matrix3x3 {
    pub m: [[f32; 3]; 3],
}

impl Div<f32> for Matrix3x3 {
    type Output = Self;

    fn div(self, determinant: f32) -> Self::Output {
        Matrix3x3 {
            m: [
                [
                    self.m[0][0] / determinant,
                    self.m[0][1] / determinant,
                    self.m[0][2] / determinant,
                ],
                [
                    self.m[1][0] / determinant,
                    self.m[1][1] / determinant,
                    self.m[1][2] / determinant,
                ],
                [
                    self.m[2][0] / determinant,
                    self.m[2][1] / determinant,
                    self.m[2][2] / determinant,
                ],
            ],
        }
    }
}

impl Mul<f32> for Matrix3x3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Matrix3x3 {
            m: [
                [
                    self.m[0][0] * scalar,
                    self.m[0][1] * scalar,
                    self.m[0][2] * scalar,
                ],
                [
                    self.m[1][0] * scalar,
                    self.m[1][1] * scalar,
                    self.m[1][2] * scalar,
                ],
                [
                    self.m[2][0] * scalar,
                    self.m[2][1] * scalar,
                    self.m[2][2] * scalar,
                ],
            ],
        }
    }
}

impl Mul for Matrix3x3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        // SAFETY because this is a 3x3 matrix we know that the resulting colums will only have 3 entries so this is safe
        let rows: Vec<Vec3> = self
            .rows()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();
        let cols: Vec<Vec3> = other
            .columns()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();

        Matrix3x3 {
            m: [
                [
                    rows[0].dot_product(&cols[0]),
                    rows[1].dot_product(&cols[0]),
                    rows[2].dot_product(&cols[0]),
                ],
                [
                    rows[0].dot_product(&cols[1]),
                    rows[1].dot_product(&cols[1]),
                    rows[2].dot_product(&cols[1]),
                ],
                [
                    rows[0].dot_product(&cols[2]),
                    rows[1].dot_product(&cols[2]),
                    rows[2].dot_product(&cols[2]),
                ],
            ],
        }
    }
}

impl<'a> Mul<&'a Matrix3x3> for Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: &'a Self) -> Self::Output {
        // SAFETY because this is a 3x3 matrix we know that the resulting colums will only have 3 entries so this is safe
        let rows: Vec<Vec3> = self
            .rows()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();
        let cols: Vec<Vec3> = other
            .columns()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();

        Matrix3x3 {
            m: [
                [
                    rows[0].dot_product(&cols[0]),
                    rows[1].dot_product(&cols[0]),
                    rows[2].dot_product(&cols[0]),
                ],
                [
                    rows[0].dot_product(&cols[1]),
                    rows[1].dot_product(&cols[1]),
                    rows[2].dot_product(&cols[1]),
                ],
                [
                    rows[0].dot_product(&cols[2]),
                    rows[1].dot_product(&cols[2]),
                    rows[2].dot_product(&cols[2]),
                ],
            ],
        }
    }
}

impl Mul<Matrix3x3> for &Matrix3x3 {
    type Output = Matrix3x3;

    fn mul(self, other: Matrix3x3) -> Self::Output {
        // SAFETY because this is a 3x3 matrix we know that the resulting colums will only have 3 entries so this is safe
        let rows: Vec<Vec3> = self
            .rows()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();
        let cols: Vec<Vec3> = other
            .columns()
            .iter()
            .map(|c| Vec3::new(c[0], c[1], c[2]))
            .collect();

        Matrix3x3 {
            m: [
                [
                    rows[0].dot_product(&cols[0]),
                    rows[1].dot_product(&cols[0]),
                    rows[2].dot_product(&cols[0]),
                ],
                [
                    rows[0].dot_product(&cols[1]),
                    rows[1].dot_product(&cols[1]),
                    rows[2].dot_product(&cols[1]),
                ],
                [
                    rows[0].dot_product(&cols[2]),
                    rows[1].dot_product(&cols[2]),
                    rows[2].dot_product(&cols[2]),
                ],
            ],
        }
    }
}

impl Matrix3x3 {
    // we always use first row first column
    pub fn calculate_determinant(&self) -> f32 {
        let det1 = self.m[0][0] * (self.m[1][1] * self.m[2][2] - self.m[1][2] * self.m[2][1]);
        let det2 = self.m[0][1] * (self.m[1][2] * self.m[2][0] - self.m[1][0] * self.m[2][2]);
        let det3 = self.m[0][2] * (self.m[1][0] * self.m[2][1] - self.m[1][1] * self.m[2][0]);

        det1 + det2 + det3
    }
    pub fn calculate_determinant_diagonal(&self) -> f32 {
        let pos1 = self.m[0][0] * self.m[1][1] * self.m[2][2];
        let pos2 = self.m[0][1] * self.m[1][2] * self.m[2][0];
        let pos3 = self.m[0][2] * self.m[1][0] * self.m[2][1];

        let neg1 = self.m[0][0] * self.m[1][2] * self.m[2][1];
        let neg2 = self.m[0][1] * self.m[1][0] * self.m[2][2];
        let neg3 = self.m[0][2] * self.m[1][1] * self.m[2][0];

        pos1 + pos2 + pos3 - neg1 - neg2 - neg3
    }
}

impl Matrix for Matrix3x3 {
    fn matrix(&self) -> &[f32] {
        bytemuck::cast_slice(&self.m)
    }
    fn identity(&self) -> Self {
        Matrix3x3 {
            m: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
        }
    }
}

impl SquareMatrix for Matrix3x3 {
    fn transpose(&self) -> Self {
        let row1 = self.row(0).unwrap();
        let row2 = self.row(1).unwrap();
        let row3 = self.row(2).unwrap();

        Matrix3x3 {
            m: [
                [row1[0], row2[0], row3[0]],
                [row1[1], row2[1], row3[1]],
                [row1[2], row2[2], row3[2]],
            ],
        }
    }
}

impl Orthogonal for Matrix3x3 {}
