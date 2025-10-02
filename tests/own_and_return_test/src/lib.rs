#[cfg(test)]
mod tests {
    use own_and_return::*;

    #[test]
    fn test_consume() {
        assert_eq!(
            take_film_name(Film {
                name: "Matrix".to_owned()
            }),
            "Matrix"
        );
    }

    #[test]
    fn test_reference() {
        assert_eq!(
            read_film_name(&Film {
                name: "Matrix".to_owned()
            }),
            "Matrix"
        );
    }
}
