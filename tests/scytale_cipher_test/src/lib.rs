use scytale_cipher::*;

#[test]
fn test_scytale_code() {
    assert_eq!(scytale_cipher("scytale Code", 6), "sec yCtoadle");
    assert_eq!(scytale_cipher("scytale Code", 8), "sCcoydtea l e");
}

#[test]
fn test_nothing() {
    assert_eq!(scytale_cipher("", 4), "");
    assert_eq!(scytale_cipher("", 0), "");
}

#[test]
fn test_same_length() {
    assert_eq!(scytale_cipher("qwerty qwerty", 13), "qwerty qwerty");
}

#[test]
fn test_thicker_cylinder() {
    assert_eq!(scytale_cipher("attack morning", 6), "a ntmgto ar cn ki");
}

#[test]
fn test_overflowing() {
    assert_eq!(scytale_cipher("abc", 6), "abc");
}

#[test]
fn test_others() {
    assert_eq!(scytale_cipher("a b c", 2), "abc");
}
