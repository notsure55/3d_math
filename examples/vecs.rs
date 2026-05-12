use anyhow::Result;
use math::{matrix_four::Matrix4x4, vec_three::Vec3};

fn main() {
    let pos = Vec3::new(90.0, 100.0, 300.0);

    let matrix = Matrix4x4 {
        m: [
            [0.5, 0.2, 0.3, 0.5],
            [0.2, 0.1, 0.24, 0.5],
            [0.4, 0.6, 0.0, 0.7],
            [0.8, 0.9, 1.0, 1.0],
        ],
    };

    let new_pos = pos * &matrix;

    println!("{new_pos:#?}");
}
