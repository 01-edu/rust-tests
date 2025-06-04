use scores::*;

#[test]
fn test_simple() {
    assert_eq!(score("a"), 1);
    assert_eq!(score("A"), 1);
    assert_eq!(score("h"), 4);
    assert_eq!(score("at"), 2);
    assert_eq!(score("Yes"), 6);
    assert_eq!(score("cellphones"), 17);
}

#[test]
fn test_empty() {
    assert_eq!(score(""), 0);
    assert_eq!(score(" "), 0);
}

#[test]
fn test_special() {
    assert_eq!(score("in Portugal NÃO stands for: no"), 30);
    assert_eq!(score("This is a test, comparação, têm Água?"), 36);
}

#[test]
fn test_long() {
    assert_eq!(score("ThiS is A Test"), 14);
    assert_eq!(score("long sentences are working"), 34);
    assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
}
