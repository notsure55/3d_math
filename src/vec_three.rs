use crate::matrix::SquareMatrix;
use crate::matrix_four::Matrix4x4;
use crate::vec_four::Vec4;
use crate::vec_two::Vec2;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot_product(&self, other: &Vec3) -> f32 {
        (self * other).sum()
    }
    pub fn cross_product(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
    pub fn triple_product(&self, other: &Vec3, other1: &Vec3) -> f32 {
        self.dot_product(&other.cross_product(other1))
    }
    pub fn sum(&self) -> f32 {
        self.x + self.y + self.z
    }
    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn normalize(&self) -> Vec3 {
        self / self.len()
    }
    pub fn angle(&self, other: &Vec3) -> f32 {
        let product = self.dot_product(other);

        let magnitude = self.len() * other.len();

        let angle = (product / magnitude).acos();
        angle.to_degrees()
    }
    // for my aimbot in cs2
    pub fn calc_view_angles(&self, target: &Vec3) -> Option<Vec2> {
        let delta = target - self;

        let yaw = delta.y.atan2(delta.x).to_degrees();

        let hypot = (delta.x * delta.x + delta.y * delta.y).sqrt();
        let pitch = hypot.atan2(delta.z).to_degrees() - 90.0;

        if yaw.is_nan() || pitch.is_nan() {
            return None;
        }

        let yaw = yaw.clamp(-180.0, 180.0);
        let pitch = pitch.clamp(-89.0, 89.0);

        Some(Vec2::new(pitch, yaw))
    }
    pub fn empty(&self) -> bool {
        if self.x == 0.0 && self.y == 0.0 && self.z == 0.0 {
            true
        } else {
            false
        }
    }
    pub fn wts(&self, view_matrix: &Matrix4x4, screen_size: (f32, f32)) -> Option<Vec2> {
        let z = view_matrix.m[3][0] * self.x
            + view_matrix.m[3][1] * self.y
            + view_matrix.m[3][2] * self.z
            + view_matrix.m[3][3];

        // if off screen
        if z < 0.001 {
            return None;
        }

        let mut out = Vec2::new(0.0, 0.0);

        out.x = screen_size.0 * 0.5;
        out.y = screen_size.1 * 0.5;

        out.x *= 1.0
            + (view_matrix.m[0][0] * self.x
                + view_matrix.m[0][1] * self.y
                + view_matrix.m[0][2] * self.z
                + view_matrix.m[0][3])
                / z;
        out.y *= 1.0
            - (view_matrix.m[1][0] * self.x
                + view_matrix.m[1][1] * self.y
                + view_matrix.m[1][2] * self.z
                + view_matrix.m[1][3])
                / z;

        Some(out)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Self::Output {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Self::Output {
        Self {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<&Matrix4x4> for Vec3 {
    type Output = Vec4;

    fn mul(self, matrix: &Matrix4x4) -> Self::Output {
        let matrix = matrix.transpose();

        Vec4 {
            x: self.x * matrix.m[0][0]
                + self.y * matrix.m[0][1]
                + self.z * matrix.m[0][2]
                + 1.0 * matrix.m[0][3],
            y: self.x * matrix.m[1][0]
                + self.y * matrix.m[1][1]
                + self.z * matrix.m[1][2]
                + 1.0 * matrix.m[1][3],
            z: self.x * matrix.m[2][0]
                + self.y * matrix.m[2][1]
                + self.z * matrix.m[2][2]
                + 1.0 * matrix.m[2][3],
            w: self.x * matrix.m[3][0]
                + self.y * matrix.m[3][1]
                + self.z * matrix.m[3][2]
                + 1.0 * matrix.m[3][3],
        }
    }
}
