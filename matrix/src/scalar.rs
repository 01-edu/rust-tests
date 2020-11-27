// A scalar type must implement the operations
// Addition, Subtraction, Multiplication and Division (you might
// also have to use more restrictions). For this use a trait
// inheritance (supertraits)

// Another condition for a number to be a scalar is to have a zero
// (neutral element in the addition) and a one (neutral element in the
// multiplication). Therefore the Scalar trait will require 2
// functions zero() and one()

// After finishing implement the Scalar trait for u32

use std::ops::{Add, Div, Mul, Sub};

pub trait Scalar: Add + Div + Mul + Sub + std::marker::Sized + Clone {
	type Item;
	fn zero() -> Self::Item;
	fn one() -> Self::Item;
}

impl Scalar for u32 {
	type Item = u32;
	fn zero() -> Self::Item {
		0 as u32
	}
	fn one() -> Self::Item {
		1 as u32
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn scalar() {
		let a: u32 = u32::zero();
		assert_eq!(a, 0 as u32);

		let b = u32::one();
		assert_eq!(b, 1 as u32);
	}
}
