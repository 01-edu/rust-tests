#[cfg(test)]
mod tests {
    use slices_to_map::*;
    use std::{collections::HashMap, sync::LazyLock};

    static EXPECTED: LazyLock<HashMap<&str, i32>> = LazyLock::new(|| {
        HashMap::from([
            ("Olivia", 1),
            ("Liam", 3),
            ("Emma", 23),
            ("Noah", 5),
            ("James", 2),
        ])
    });

    #[test]
    fn test_same_length() {
        assert_eq!(
            slices_to_map(
                &["Olivia", "Liam", "Emma", "Noah", "James"],
                &[1, 3, 23, 5, 2]
            ),
            *EXPECTED
        );
    }

    #[test]
    fn test_different_length() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2, 9];
        assert_eq!(slices_to_map(&keys, &values), *EXPECTED);

        let keys = ["Olivia", "Liam", "Emma", "Noah", "James", "Isabella"];
        let values = [1, 3, 23, 5, 2];
        assert_eq!(slices_to_map(&keys, &values), *EXPECTED);
    }

    #[test]
    fn it_works_for_vecs() {
        let keys = ["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = [1, 3, 23, 5, 2];
        assert_eq!(slices_to_map(&keys, &values), *EXPECTED);

        let keys = vec!["Olivia", "Liam", "Emma", "Noah", "James"];
        let values = vec![1, 3, 23, 5, 2, 9];
        assert_eq!(slices_to_map(&keys, &values), *EXPECTED);

        let keys = vec!["Olivia", "Liam", "Emma", "Noah", "James", "Isabella"];
        let values = vec![1, 3, 23, 5, 2];
        assert_eq!(slices_to_map(&keys, &values), *EXPECTED);
    }
}
