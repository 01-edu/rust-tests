#[derive(Debug, PartialEq, Eq)]
pub struct Matrix(pub (i32, i32), pub (i32, i32));

pub fn multiply(m: Matrix, val: i32) -> Matrix {
    Matrix(
        ((m.0).0 * val, (m.0).1 * val),
        ((m.1).0 * val, (m.1).1 * val),
    )
}
