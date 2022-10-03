// In this exercise you will define the basic operations with a matrix
// starting by implementing the `std::ops::Add` trait

// Define the operation + (by defining the trait std::ops::Add) for
// two matrices remember that two matrices can only be added if they
// have the same size. Therefore the add method must handle the
// possibility of failure by returning an Option<T>

use crate::{Matrix, Scalar};
use std::ops::Add;

impl<T: Scalar<Item = T> + std::ops::Add<Output = T>> Add for Matrix<T> {
    type Output = Option<Self>;

    fn add(self, other: Self) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() + other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}

use std::ops::Sub;

impl<T: Scalar<Item = T> + Sub<Output = T>> Sub for Matrix<T> {
    type Output = Option<Self>;

    fn sub(self, other: Self) -> Self::Output {
        if self.0[0].len() != other.0[0].len() || self.0.len() != other.0.len() {
            return None;
        }

        let mut matrix = Matrix::new();
        for (j, row) in self.0.iter().enumerate() {
            if j > 0 {
                matrix.0.push(Vec::new());
            }
            for (i, v) in row.iter().enumerate() {
                matrix.0[j].push(v.clone() - other.0[j][i].clone());
            }
        }

        Some(matrix)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn addition() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![2, 2], vec![2, 2]]);
        assert_eq!(matrix + matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix + matrix_2, None);
    }

    #[test]
    fn subtraction() {
        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let expected = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix - matrix_2, Some(expected));

        let matrix = Matrix(vec![vec![1, 1], vec![1, 1]]);
        let matrix_2 = Matrix(vec![vec![1, 1, 3], vec![1, 1]]);
        assert_eq!(matrix - matrix_2, None);
    }
}
