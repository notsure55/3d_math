use super::matrix::{Matrix, SquareMatrix};
use super::matrix_three::Matrix3x3;
use std::ops::{Div, Mul};

use super::vec_three::Vec3;
use super::vec_two::Vec2;

#[derive(Debug, Default, PartialEq)]
pub struct Matrix4x4 {
    pub m: [[f32; 4]; 4],
}

impl Mul for Matrix4x4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let other = other.transpose();
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] * other.m[0][0]
                        + self.m[0][1] * other.m[0][1]
                        + self.m[0][2] * other.m[0][2]
                        + self.m[0][3] * other.m[0][3],
                    self.m[0][0] * other.m[1][0]
                        + self.m[0][1] * other.m[1][1]
                        + self.m[0][2] * other.m[1][2]
                        + self.m[0][3] * other.m[1][3],
                    self.m[0][0] * other.m[2][0]
                        + self.m[0][1] * other.m[2][1]
                        + self.m[0][2] * other.m[2][2]
                        + self.m[0][3] * other.m[3][3],
                    self.m[0][0] * other.m[3][0]
                        + self.m[0][1] * other.m[3][1]
                        + self.m[0][2] * other.m[3][2]
                        + self.m[0][3] * other.m[3][3],
                ],
                [
                    self.m[1][0] * other.m[0][0]
                        + self.m[1][1] * other.m[0][1]
                        + self.m[1][2] * other.m[0][2]
                        + self.m[1][3] * other.m[0][3],
                    self.m[1][0] * other.m[1][0]
                        + self.m[1][1] * other.m[1][1]
                        + self.m[1][2] * other.m[1][2]
                        + self.m[1][3] * other.m[1][3],
                    self.m[1][0] * other.m[2][0]
                        + self.m[1][1] * other.m[2][1]
                        + self.m[1][2] * other.m[2][2]
                        + self.m[1][3] * other.m[3][3],
                    self.m[1][0] * other.m[3][0]
                        + self.m[1][1] * other.m[3][1]
                        + self.m[1][2] * other.m[3][2]
                        + self.m[1][3] * other.m[3][3],
                ],
                [
                    self.m[2][0] * other.m[0][0]
                        + self.m[2][1] * other.m[0][1]
                        + self.m[2][2] * other.m[0][2]
                        + self.m[2][3] * other.m[0][3],
                    self.m[2][0] * other.m[1][0]
                        + self.m[2][1] * other.m[1][1]
                        + self.m[2][2] * other.m[1][2]
                        + self.m[2][3] * other.m[1][3],
                    self.m[2][0] * other.m[2][0]
                        + self.m[2][1] * other.m[2][1]
                        + self.m[2][2] * other.m[2][2]
                        + self.m[2][3] * other.m[3][3],
                    self.m[2][0] * other.m[3][0]
                        + self.m[2][1] * other.m[3][1]
                        + self.m[2][2] * other.m[3][2]
                        + self.m[2][3] * other.m[3][3],
                ],
                [
                    self.m[3][0] * other.m[0][0]
                        + self.m[3][1] * other.m[0][1]
                        + self.m[3][2] * other.m[0][2]
                        + self.m[3][3] * other.m[0][3],
                    self.m[3][0] * other.m[1][0]
                        + self.m[3][1] * other.m[1][1]
                        + self.m[3][2] * other.m[1][2]
                        + self.m[3][3] * other.m[1][3],
                    self.m[3][0] * other.m[2][0]
                        + self.m[3][1] * other.m[2][1]
                        + self.m[3][2] * other.m[2][2]
                        + self.m[3][3] * other.m[3][3],
                    self.m[3][0] * other.m[3][0]
                        + self.m[3][1] * other.m[3][1]
                        + self.m[3][2] * other.m[3][2]
                        + self.m[3][3] * other.m[3][3],
                ],
            ],
        }
    }
}

impl<'a> Mul<&'a Matrix4x4> for Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: &'a Self) -> Self::Output {
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] * other.m[0][0],
                    self.m[0][1] * other.m[0][1],
                    self.m[0][2] * other.m[0][2],
                    self.m[0][3] * other.m[0][3],
                ],
                [
                    self.m[1][0] * other.m[1][0],
                    self.m[1][1] * other.m[1][1],
                    self.m[1][2] * other.m[1][2],
                    self.m[1][3] * other.m[1][3],
                ],
                [
                    self.m[2][0] * other.m[2][0],
                    self.m[2][1] * other.m[2][1],
                    self.m[2][2] * other.m[2][2],
                    self.m[2][3] * other.m[2][3],
                ],
                [
                    self.m[3][0] * other.m[3][0],
                    self.m[3][1] * other.m[3][1],
                    self.m[3][2] * other.m[3][2],
                    self.m[3][3] * other.m[3][3],
                ],
            ],
        }
    }
}

impl Mul<f32> for Matrix4x4 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] * scalar,
                    self.m[0][1] * scalar,
                    self.m[0][2] * scalar,
                    self.m[0][3] * scalar,
                ],
                [
                    self.m[1][0] * scalar,
                    self.m[1][1] * scalar,
                    self.m[1][2] * scalar,
                    self.m[1][3] * scalar,
                ],
                [
                    self.m[2][0] * scalar,
                    self.m[2][1] * scalar,
                    self.m[2][2] * scalar,
                    self.m[2][3] * scalar,
                ],
                [
                    self.m[3][0] * scalar,
                    self.m[3][1] * scalar,
                    self.m[3][2] * scalar,
                    self.m[3][3] * scalar,
                ],
            ],
        }
    }
}

impl Div<f32> for Matrix4x4 {
    type Output = Self;

    fn div(self, determinant: f32) -> Self::Output {
        Matrix4x4 {
            m: [
                [
                    self.m[0][0] / determinant,
                    self.m[0][1] / determinant,
                    self.m[0][2] / determinant,
                    self.m[0][3] / determinant,
                ],
                [
                    self.m[1][0] / determinant,
                    self.m[1][1] / determinant,
                    self.m[1][2] / determinant,
                    self.m[1][3] / determinant,
                ],
                [
                    self.m[2][0] / determinant,
                    self.m[2][1] / determinant,
                    self.m[2][2] / determinant,
                    self.m[2][3] / determinant,
                ],
                [
                    self.m[3][0] / determinant,
                    self.m[3][1] / determinant,
                    self.m[3][2] / determinant,
                    self.m[3][3] / determinant,
                ],
            ],
        }
    }
}

impl Matrix4x4 {
    pub fn calculate_determinant(&self) -> f32 {
        let mut minors = Vec::with_capacity(4);

        for col in 0..self.m[0].len() {
            let minor = self.minor(0, col);
            minors.push(minor);
        }

        //let three_x_threes = Matrix3x3::from_matrix4x4(self);
        let mut final_det: f32 = 0.0;

        for (i, (cofactor, mat)) in minors.iter().enumerate() {
            let determinant = mat.calculate_determinant();

            if (i + 1) % 2 != 0 {
                final_det += cofactor * determinant;
            } else {
                final_det -= cofactor * determinant;
            }
        }

        final_det
    }
    pub fn calculate_determinant_c4_minus_c1(&self) -> f32 {
        let c1 = self.col(0).unwrap();
        let c4 = self.col(3).unwrap();
        let c4 = [c4[0] - c1[0], c4[1] - c1[1], c4[2] - c1[2], c4[3] - c1[3]];

        let cofactor_matrix = Matrix3x3 {
            m: [
                [self.m[1][1], self.m[1][2], c4[1]],
                [self.m[2][1], self.m[2][2], c4[2]],
                [self.m[3][1], self.m[3][2], c4[3]],
            ],
        };

        cofactor_matrix.calculate_determinant_diagonal()
    }
    pub fn minor(&self, ro: usize, col: usize) -> (f32, Matrix3x3) {
        let cofactor = self.m[ro][col];

        let mut positions = vec![];

        for row in 0..self.m.len() {
            for column in 0..self.m[row].len() {
                if row == ro {
                    break;
                }
                if column == col {
                    continue;
                }

                positions.push(self.m[row][column]);
            }
        }

        assert!(positions.len() == 9);

        (
            cofactor,
            Matrix3x3 {
                m: [
                    [positions[0], positions[1], positions[2]],
                    [positions[3], positions[4], positions[5]],
                    [positions[6], positions[7], positions[8]],
                ],
            },
        )
    }
    pub fn calculate_adjugate(&self) -> Matrix4x4 {
        let mut matrix = Matrix4x4::default();

        let mut plus = true;

        for row in 0..4 {
            for col in 0..4 {
                let (_, minor) = self.minor(row, col);
                let determinant = minor.calculate_determinant();
                matrix.m[row][col] = if plus { determinant } else { -determinant };

                if col != 3 {
                    plus = !plus;
                }
            }
        }

        matrix
    }
    pub fn inverse(&self) -> Matrix4x4 {
        let determ = self.calculate_determinant();

        let matrix = self.calculate_adjugate();

        let transposed = matrix.transpose();

        transposed / determ
    }
}

impl Matrix for Matrix4x4 {
    fn matrix(&self) -> &[f32] {
        bytemuck::cast_slice(&self.m)
    }
    fn identity(&self) -> Self {
        Matrix4x4 {
            m: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }
}

impl SquareMatrix for Matrix4x4 {
    fn transpose(&self) -> Self {
        let row1 = self.row(0).unwrap();
        let row2 = self.row(1).unwrap();
        let row3 = self.row(2).unwrap();
        let row4 = self.row(3).unwrap();

        Matrix4x4 {
            m: [
                [row1[0], row2[0], row3[0], row4[0]],
                [row1[1], row2[1], row3[1], row4[1]],
                [row1[2], row2[2], row3[2], row4[2]],
                [row1[3], row2[3], row3[3], row4[3]],
            ],
        }
    }
}
