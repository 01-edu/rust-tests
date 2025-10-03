use std::ops::{Add, Mul, Sub};

use lalgebra_scalar::Scalar;
use matrix::Matrix;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W: usize, const H: usize, T>(pub Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
    #[inline]
    fn from(array: [[T; W]; H]) -> Self {
        Wrapper(Matrix(array))
    }
}

macro_rules! impl_binary_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl<const W: usize, const H: usize, T: Scalar<Item = T> + $trait<Output = T>> $trait for Wrapper<W, H, T> {
            type Output = Self;

            #[inline]
            fn $method(self, other: Self) -> Self::Output {
                std::array::from_fn(|j| {
                    std::array::from_fn(|i| self.0.0[j][i] $op other.0.0[j][i])
                }).into()
            }
        }
    };
}

impl_binary_op!(Add, add, +);
impl_binary_op!(Sub, sub, -);

impl<const S: usize, T: Scalar<Item = T> + std::iter::Sum<<T as Mul>::Output>> Mul
    for Wrapper<S, S, T>
{
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self::Output {
        std::array::from_fn(|j| {
            std::array::from_fn(|i| (0..S).map(|k| self.0.0[j][k] * rhs.0.0[k][i]).sum())
        })
        .into()
    }
}
