// A vector in linear algebra is define as "anything that can be added
// and that can be multiplied by a scalar"
// And the associated function dot that calculates the dot product
// between two vectors
// let vector = Vector(vec![0,3, 4]);
// let vector_1 = Vector(vec![0,3,3]);
// vector.dot(&vector_1) == Some(23);

// The dot product between two vectors of different length it's not defined

use lalgebra_scalar::Scalar;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Vector<T: Scalar<Item = T>>(pub Vec<T>);

use std::ops::Add;

impl<T: Scalar<Item = T> + Add<Output = T>> Add<Self> for Vector<T> {
    type Output = Option<Self>;
    fn add(self, other: Self) -> Self::Output {
        if self.0.len() != other.0.len() {
            return None;
        }

        let result: Vector<T> = Vector(
            self.0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.clone() + y.clone())
                .collect(),
        );
        Some(result)
    }
}

impl<T: Scalar<Item = T> + std::iter::Sum<<T as std::ops::Mul>::Output>> Vector<T> {
    pub fn new() -> Self {
        Vector(Vec::new())
    }

    pub fn dot(&self, other: &Self) -> Option<T> {
        if self.0.len() != other.0.len() {
            return None;
        }
        let result = self
            .0
            .iter()
            .zip(other.0.iter())
            .map(|(x, y)| x.clone() * y.clone())
            .sum();

        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot_product() {
        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2, -1]);
        let expected: i64 = 3;
        assert_eq!(vector_1.dot(&vector_2), Some(expected));

        let vector_1: Vector<i64> = Vector(vec![1, 3, -5]);
        let vector_2: Vector<i64> = Vector(vec![4, -2]);
        assert_eq!(vector_1.dot(&vector_2), None);
    }
}
