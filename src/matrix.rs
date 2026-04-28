use smallvec::SmallVec;
use std::fmt::Debug;
use std::ops::{Div, Mul};

// TODO research trait bounds more indepth, this is probaly a bad implementation
pub trait Matrix:
    Div<f32> + Mul<f32> + Mul<Self> + for<'a> Mul<&'a Self> + PartialEq<Self> + Sized + Debug
{
    fn matrix(&self) -> &[f32];
    fn identity(&self) -> Self;
}

// TODO figure out a better way than smallvec for stack dynamic stack allocation
pub trait SquareMatrix: Matrix {
    fn row(&self, index: usize) -> Option<SmallVec<[f32; 4]>> {
        let matrix = self.matrix();
        let matrix_len = matrix.len().isqrt();

        if index > matrix_len {
            None
        } else {
            let mut vec: SmallVec<[f32; 4]> = SmallVec::with_capacity(matrix_len);
            for i in 0..matrix_len {
                vec.push(matrix[i + index * matrix_len])
            }
            Some(vec)
        }
    }
    fn col(&self, index: usize) -> Option<SmallVec<[f32; 4]>> {
        let matrix = self.matrix();
        let matrix_len = matrix.len().isqrt();

        if index > matrix_len {
            None
        } else {
            let mut vec: SmallVec<[f32; 4]> = SmallVec::with_capacity(matrix_len);

            for i in 0..matrix_len {
                vec.push(matrix[i * matrix_len + index])
            }

            Some(vec)
        }
    }
    fn columns(&self) -> SmallVec<[SmallVec<[f32; 4]>; 4]> {
        let matrix = self.matrix();
        let matrix_len = matrix.len().isqrt();
        let mut columns: SmallVec<[SmallVec<[f32; 4]>; 4]> = SmallVec::with_capacity(matrix_len);

        for i in 0..matrix_len {
            let column = self.col(i).unwrap();
            columns.push(column);
        }

        columns
    }
    fn rows(&self) -> SmallVec<[SmallVec<[f32; 4]>; 4]> {
        let matrix = self.matrix();
        let matrix_len = matrix.len().isqrt();
        let mut rows: SmallVec<[SmallVec<[f32; 4]>; 4]> = SmallVec::with_capacity(matrix_len);

        for i in 0..matrix_len {
            let row = self.row(i).unwrap();
            rows.push(row);
        }

        rows
    }
    fn transpose(&self) -> Self;
}

pub trait Orthogonal: SquareMatrix {
    fn is_orthogonal(&self) -> bool
    where
        for<'a> <Self as Mul<&'a Self>>::Output: PartialEq<Self>,
        for<'a> <Self as Mul<&'a Self>>::Output: Debug,
    {
        let transposed = self.transpose();
        let maybe_identity = transposed * self;
        let identity = self.identity();

        println!("{:?}, {:?}", &maybe_identity, &identity);

        // TODO fix partial eq to compare floats properly
        if maybe_identity == identity {
            true
        } else {
            false
        }
    }
}
