use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Sized + Add + Sub + Mul + Div + Clone + Copy {
    type Item;

    fn zero() -> Self::Item;
    fn one() -> Self::Item;
}

impl Scalar for u32 {
    type Item = u32;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for u64 {
    type Item = u64;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i32 {
    type Item = i32;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for i64 {
    type Item = i64;

    #[inline]
    fn zero() -> Self::Item {
        0
    }

    #[inline]
    fn one() -> Self::Item {
        1
    }
}

impl Scalar for f32 {
    type Item = f32;

    #[inline]
    fn zero() -> Self::Item {
        0.
    }

    #[inline]
    fn one() -> Self::Item {
        1.
    }
}

impl Scalar for f64 {
    type Item = f64;

    #[inline]
    fn zero() -> Self::Item {
        0.
    }

    #[inline]
    fn one() -> Self::Item {
        1.
    }
}
