use lalgebra_scalar::Scalar;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T: Scalar<Item = T>> Matrix<W, H, T> {
    #[inline]
    pub fn zero() -> Self {
        Matrix([[T::zero(); W]; H])
    }
}

impl<const S: usize, T: Scalar<Item = T>> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut matrix = Matrix::zero();
        for i in 0..S {
            matrix.0[i][i] = T::one();
        }
        matrix
    }
}
