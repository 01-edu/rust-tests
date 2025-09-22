use counting_words::*;

fn cmp_hashmap_unordered(input: &str, expected: &[(&str, u32)]) {
    let m = counting_words(input);

    assert_eq!(expected.len(), m.len());
    expected
        .iter()
        .for_each(|&(k, v)| assert_eq!(m.get(k), Some(&v)));
}

#[test]
fn test_simple() {
    cmp_hashmap_unordered("word", &[("word", 1)]);
    cmp_hashmap_unordered("hello", &[("hello", 1)]);
    cmp_hashmap_unordered("hello big world", &[("hello", 1), ("big", 1), ("world", 1)]);
    cmp_hashmap_unordered("one of each", &[("one", 1), ("of", 1), ("each", 1)]);
    cmp_hashmap_unordered("Hello, 1, 2 HELLO", &[("hello", 2), ("1", 1), ("2", 1)]);
    cmp_hashmap_unordered(
        "Batman, BATMAN, batman, Stop stop",
        &[("batman", 3), ("stop", 2)],
    );
    cmp_hashmap_unordered(
        " multiple   whitespace",
        &[("multiple", 1), ("whitespace", 1)],
    );
}

#[test]
fn test_count_multiple_occurrences() {
    cmp_hashmap_unordered(
        "one fish two fish red fish blue fish",
        &[("one", 1), ("fish", 4), ("two", 1), ("red", 1), ("blue", 1)],
    );
}

#[test]
fn test_multi_lines() {
    cmp_hashmap_unordered(
        "Game\nNight\nTomorrow",
        &[("game", 1), ("night", 1), ("tomorrow", 1)],
    );
}

#[test]
fn test_punctuation() {
    cmp_hashmap_unordered(
        "keyboard : mouse on the desk : Computer!!&@$%^&",
        &[
            ("keyboard", 1),
            ("mouse", 1),
            ("on", 1),
            ("the", 1),
            ("desk", 1),
            ("computer", 1),
        ],
    );
}

#[test]
fn with_apostrophes() {
    cmp_hashmap_unordered(
        "First: don't laugh. Then: don't cry.",
        &[
            ("first", 1),
            ("don't", 2),
            ("laugh", 1),
            ("then", 1),
            ("cry", 1),
        ],
    );
}

#[test]
fn test_apostrophe() {
    cmp_hashmap_unordered(
        "Joe can't tell between 'large' and large.",
        &[
            ("joe", 1),
            ("can't", 1),
            ("tell", 1),
            ("between", 1),
            ("large", 2),
            ("and", 1),
        ],
    );
}
