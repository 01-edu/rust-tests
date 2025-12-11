#[cfg(test)]
mod tests {
    use get_products::*;

    #[test]
    fn test_multiple() {
        assert_eq!(get_products([1, 7, 3, 4]), [84, 12, 28, 21]);
        assert_eq!(get_products([10, 3, 5, 6, 2]), [180, 600, 360, 300, 900]);
    }

    #[test]
    fn test_empty_case() {
        assert_eq!(get_products([]), []);
    }

    #[test]
    fn test_single_case() {
        assert_eq!(get_products([99]), [1]);
    }
}
