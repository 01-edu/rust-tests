// First exercise

// # Instructions
// Define a data structure to represent a matrix of any size and
// implement the basic operations for this you will need to follow the
// next steps:

// We will consider a matrix as a rectangular arrangements of scalars
// Therefore you will have to create the definition of a Scalar by
// defining a trait Scalar.

// A scalar type must implement the operations
// Addition, Subtraction, Multiplication, Modulo, Division (you might
// also have to use more restrictions). For this use a trait
// inheritance (supertraits)

// Another condition for a number to be a scalar is to have a zero
// (neutral element in the addition) and a one (neutral element in the
// multiplication). Therefore the Scalar trait will require 2
// functions zero() and one()

// After finishing implement the Scalar trait for u32

// Second exercise

// Continuing with this library we will define the Matrix<T> type
// we will define it as a wrapper for a Vec of Vecs (To make it flexible)
// That is, as a two dimensional Vec

// Let's start to define functions for our matrices

// First define an associated function call `new` that returns an
// empty matrix
// then define the associated function zero(row, col) that returns
// a matrix of size `row x col` (row by col) with all positions filled
// with the zero of each type

// Then define the function identity that returns the identity matrix
// of size `row x col`

// Resources: https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

use std::ops::{Add, Div, Mul, Rem, Sub};

trait Scalar: Add + Div + Mul + Rem + Sub + std::marker::Sized + Clone {
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

#[derive(Debug, Eq, PartialEq)]
struct Matrix<T>(Vec<Vec<T>>);

impl<T: Scalar<Item = T>> Matrix<T> {
	fn new() -> Matrix<T> {
		Matrix(vec![Vec::new()])
	}
	// It returns the zero matrix of the size given by the row and
	// column parameters
	fn zero(row: usize, col: usize) -> Matrix<T> {
		let mut matrix = Matrix(Vec::new());
		for _ in 0..row {
			matrix.0.push(vec![T::zero(); col]);
		}
		matrix
	}

	fn identity(n: usize) -> Matrix<T> {
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
	fn scalar() {
		let a: u32 = u32::zero();
		assert_eq!(a, 0 as u32);

		let b = u32::one();
		assert_eq!(b, 1 as u32);
	}

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
