#[cfg(test)]
mod tests {
    use macro_map::hash_map;
    use std::collections::HashMap;

    #[test]
    fn empty() {
        let expected = HashMap::<u32, u32>::new();
        let computed = hash_map!();

        assert_eq!(computed, expected);
    }

    #[test]
    fn one_element() {
        let expected = HashMap::from([("my name", 10)]);
        let computed = hash_map!("my name" => 10);

        assert_eq!(computed, expected);
    }

    #[test]
    fn multiple_elements_one_line() {
        let expected = HashMap::from([("my name", 10), ("another name", 22)]);
        let computed = hash_map!("my name" => 10, "another name" => 22);

        assert_eq!(computed, expected);
    }

    #[test]
    fn multiple_elements_multiple_lines() {
        let expected =
            HashMap::from([("my name", 10), ("another name", 22), ("the third one", 33)]);
        let computed = hash_map!(
            "my name" => 10,
            "another name" => 22,
            "the third one" => 33,
        );

        assert_eq!(computed, expected);
    }
}
