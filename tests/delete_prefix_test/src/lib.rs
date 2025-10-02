#[cfg(test)]
mod tests {
    use delete_prefix::*;

    #[test]
    fn test_delete_prefix() {
        assert_eq!(delete_prefix("john", "john wick"), Some(" wick"));

        assert_eq!(delete_prefix("ab", "b"), None);

        assert_eq!(delete_prefix("aa", "ab"), None);

        assert_eq!(delete_prefix("á©", "á©ab"), Some("ab"));
    }
}
