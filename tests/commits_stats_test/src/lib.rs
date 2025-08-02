use commits_stats::*;
use std::collections::HashMap;

#[inline]
fn data() -> json::JsonValue {
    json::parse(include_str!("../commits.json")).unwrap()
}

#[test]
fn test_commits_per_week() {
    assert_eq!(
        commits_per_week(&data()),
        HashMap::from([
            ("2020-W47".to_string(), 3),
            ("2020-W43".to_string(), 1),
            ("2020-W36".to_string(), 1),
            ("2020-W50".to_string(), 2),
            ("2020-W40".to_string(), 2),
            ("2020-W44".to_string(), 5),
            ("2020-W46".to_string(), 4),
            ("2020-W31".to_string(), 1),
            ("2020-W45".to_string(), 4),
            ("2020-W49".to_string(), 7),
        ])
    );
}

#[test]
fn test_commits_per_author() {
    assert_eq!(
        commits_per_author(&data()),
        HashMap::from([
            ("RPigott".to_string(), 1),
            ("RedSoxFan".to_string(), 1),
            ("Xyene".to_string(), 7),
            ("paul-ri".to_string(), 2),
            ("JayceFayne".to_string(), 1),
            ("mwenzkowski".to_string(), 3),
            ("psnszsn".to_string(), 1),
            ("emersion".to_string(), 10),
            ("tamirzb".to_string(), 1),
            ("ifreund".to_string(), 1),
            ("homembaixinho".to_string(), 2),
        ])
    )
}
