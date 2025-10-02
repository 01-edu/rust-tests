#[cfg(test)]
mod tests {
    use flat_tree::*;
    use std::collections::BTreeSet;

    #[test]
    fn it_works() {
        assert_eq!(flatten_tree(&BTreeSet::from([3, 0, 9, 10])), [0, 3, 9, 10]);
    }

    #[test]
    fn test_with_str() {
        assert_eq!(
            flatten_tree(&BTreeSet::from(["Slow", "kill", "will", "Horses"])),
            ["Horses", "Slow", "kill", "will"]
        );
    }
}
