use std::ops::Not;

use luhn_algorithm::*;

#[test]
fn test_subject_examples() {
    assert!(is_luhn_formula("").not(), "An empty string is not valid");
    assert!(is_luhn_formula("1").not(), "1 is not valid");
    assert!(is_luhn_formula("79927398713"), "79927398713 is valid");
}

#[test]
fn test_valid_numbers() {
    assert!(
        is_luhn_formula("4532015112830366"),
        "4532015112830366 is valid"
    );
    assert!(
        is_luhn_formula("6011 1111 1111 1117"),
        "6011 1111 1111 1117 is valid"
    );
    assert!(
        is_luhn_formula("371449635398431"),
        "371449635398431 is valid"
    );
    assert!(
        is_luhn_formula("  5555555555554444"),
        "  5555555555554444 is valid"
    );
    assert!(
        is_luhn_formula("79927398713 8336263  "),
        "79927398713 8336263   is valid"
    );
}

#[test]
fn test_invalid_numbers() {
    assert!(
        is_luhn_formula("1234567890123456").not(),
        "1234567890123456 is not valid"
    );
    assert!(
        is_luhn_formula("a6011 1111 1111 1117x").not(),
        "a6011 1111 1111 1117x is not valid"
    );
    assert!(
        is_luhn_formula("4444333322221110").not(),
        "4444333322221110 is not valid"
    );
    assert!(
        is_luhn_formula("9876543210987654").not(),
        "9876543210987654 is not valid"
    );
    assert!(
        is_luhn_formula("8765432198765432").not(),
        "8765432198765432 is not valid"
    );
    assert!(
        is_luhn_formula("1111222233334445").not(),
        "1111222233334445 is not valid"
    );
}
