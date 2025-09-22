use smallest::*;
use std::collections::HashMap;

#[test]
fn test_positive() {
    let f = HashMap::from([
        ("Cat", 12),
        ("Dog", 333),
        ("Elephant", 334),
        ("Gorilla", 14),
        ("Dolphin", 2),
    ]);

    assert_eq!(2, smallest(f));
}

#[test]
fn test_negative() {
    let f = HashMap::from([
        ("Daniel", 41758712),
        ("Ashley", 54551444),
        ("Katie", 575556334),
        ("Roberti", 574148),
        ("Robert", -4),
    ]);

    assert_eq!(-4, smallest(f));
}

#[test]
fn test_zero() {
    let f = HashMap::from([
        ("Mars", 1223),
        ("Jupiter", 33343),
        ("Saturn", 12443334),
        ("Neptune", 14),
        ("Venus", 0),
    ]);

    assert_eq!(0, smallest(f));
}

#[test]
fn empty() {
    let f = HashMap::new();

    assert_eq!(i32::MAX, smallest(f));
}
