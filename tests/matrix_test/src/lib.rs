#[cfg(test)]
mod tests {
    use matrix::Matrix;

    #[test]
    fn zero_property() {
        let matrix = Matrix::zero();
        let expected = Matrix([[0; 4]; 3]);
        assert_eq!(matrix, expected);

        let matrix = Matrix::zero();
        let expected = Matrix([[0; 2]; 2]);
        assert_eq!(matrix, expected);
    }

    #[test]
    fn identity_matrix() {
        let matrix = Matrix::identity();
        let expected = Matrix([[1, 0], [0, 1]]);
        assert_eq!(matrix, expected);

        let matrix = Matrix::identity();
        let expected = Matrix([[1, 0, 0], [0, 1, 0], [0, 0, 1]]);
        assert_eq!(matrix, expected);
    }
}
