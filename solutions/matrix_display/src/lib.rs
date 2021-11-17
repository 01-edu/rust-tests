// Implement the std::fmt::Display trait for a matrix of i32
// using the struct Matrix define the associated function new that
// creates a new Matrix from &[&[i32]]
// After implement the std::fmt::Display trait to print the matrix
// like this

// ```
// (1 2 3)
// (4 5 6)
// (7 8 9)
// ```

#[derive(Debug, Clone)]
pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut inner = Vec::new();
        for row in slice {
            inner.push(row.to_vec());
        }
        Self(inner)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (i, row) in self.0.iter().enumerate() {
            write!(f, "(")?;
            for (j, el) in row.iter().enumerate() {
                write!(f, "{}", el)?;
                if j != row.len() - 1 {
                    write!(f, " ")?;
                }
            }
            write!(f, ")")?;
            if i != self.0.len() - 1 {
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let matrix = Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]);
        let display = String::from("(1 2 3)\n(4 5 6)\n(7 8 9)");
        assert_eq!(display, matrix.to_string());
    }

    #[test]
    fn test_matrix_col() {
        let matrix = Matrix::new(&[&[1], &[2], &[3]]);
        let display = String::from("(1)\n(2)\n(3)");
        assert_eq!(matrix.to_string(), display);
    }
}
