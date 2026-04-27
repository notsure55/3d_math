use anyhow::Result;
use math::matrix::*;
use math::matrix_four::*;
use math::matrix_three::*;

fn main() -> Result<()> {
    let mat4x4 = Matrix4x4 {
        m: [
            [6.0, -2.0, -2.0, 1.0],
            [9.0, 1.0, 13.0, 1.0],
            [0.0, -8.0, -8.0, 1.0],
            [1.0, 2.0, 3.0, 1.0],
        ],
    };

    let mat3x3 = Matrix3x3 {
        m: [
            [1.0 / 3.0, 2.0 / 3.0, -2.0 / 3.0],
            [-2.0 / 3.0, 2.0 / 3.0, 1.0 / 3.0],
            [2.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0],
        ],
    };

    let is_orthogonal = mat3x3.is_orthogonal();

    dbg!(is_orthogonal);

    Ok(())
}
