#[cfg(test)]
mod tests {
    use matrix_ops::Wrapper;

    #[test]
    fn add() {
        assert_eq!(
            Wrapper::from([[1, 1], [1, 1]]) + Wrapper::from([[1, 1], [1, 1]]),
            Wrapper::from([[2, 2], [2, 2]])
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Wrapper::from([[1, 1], [1, 1]]) - Wrapper::from([[1, 1], [1, 1]]),
            Wrapper::from([[0, 0], [0, 0]])
        );
    }

    #[test]
    fn mul() {
        assert_eq!(
            Wrapper::from([[1, 2], [3, 4]]) * Wrapper::from([[5, 6], [7, 8]]),
            Wrapper::from([[19, 22], [43, 50]])
        );
    }
}
