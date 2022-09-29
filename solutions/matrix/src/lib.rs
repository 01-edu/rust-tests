// First exercise

// # Instructions
// Define a data structure to represent a matrix of any size and
// implement the basic operations for this you will need to follow the
// next steps:

// You can use a 2 dimensional Vec<T>'s
// We will consider a matrix as a rectangular arrangements of scalars
// You can use the definition of scalars done in the last exercise:
// `lalgebra_scalar`

// Then define the associated function `identity` that returns the identity matrix
// of size n
// Ex:
// Matrix::identity(3) == [[1,0,0], [0,1,0], [0,0,1]]

// And the associated function `zero` that returns a matrix of size
// `row x col` with all the positions filled by zeroes
// Ex:
// Matrix::zero(3, 3) == [[0,0,0],[0,0,0],[0,0,0]]

// Resources: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

use lalgebra_scalar::Scalar;
mod mult;
mod ops;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Matrix<T>(pub Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
    pub fn new() -> Matrix<T> {
        Matrix(vec![Vec::new()])
    }
    // It returns the zero matrix of the size given by the row and
    // column parameters
    pub fn zero(row: usize, col: usize) -> Matrix<T> {
        let mut matrix = Matrix(Vec::new());
        for _ in 0..row {
            matrix.0.push(vec![T::zero(); col]);
        }
        matrix
    }

    pub fn identity(n: usize) -> Matrix<T> {
        let mut matrix = Matrix::new();
        for y in 0..n {
            if y > 0 {
                matrix.0.push(Vec::new());
            }
            for x in 0..n {
                if y == x {
                    matrix.0[y].push(T::one());
                } else {
                    matrix.0[y].push(T::zero());
                }
            }
        }
        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_property() {
        let matrix: Matrix<u32> = Matrix::zero(3, 4);
        let expected: Matrix<u32> =
            Matrix(vec![vec![0, 0, 0, 0], vec![0, 0, 0, 0], vec![0, 0, 0, 0]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::zero(2, 2);
        let expected: Matrix<u32> = Matrix(vec![vec![0, 0], vec![0, 0]]);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn identy_matrix() {
        let matrix: Matrix<u32> = Matrix::identity(2);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0], vec![0, 1]]);
        assert_eq!(matrix, expected);

        let matrix: Matrix<u32> = Matrix::identity(3);
        let expected: Matrix<u32> = Matrix(vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]]);
        assert_eq!(matrix, expected);
    }
}
