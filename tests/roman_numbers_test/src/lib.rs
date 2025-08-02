use roman_numbers::*;

#[test]
fn it_works() {
    use roman_numbers::RomanDigit::*;

    assert_eq!(RomanNumber::from(0).0, [Nulla]);
    assert_eq!(RomanNumber::from(3).0, [I, I, I]);
    assert_eq!(RomanNumber::from(6).0, [V, I]);
    assert_eq!(RomanNumber::from(15).0, [X, V]);
    assert_eq!(RomanNumber::from(30).0, [X, X, X]);
    assert_eq!(RomanNumber::from(150).0, [C, L]);
    assert_eq!(RomanNumber::from(200).0, [C, C]);
    assert_eq!(RomanNumber::from(600).0, [D, C]);
    assert_eq!(RomanNumber::from(1500).0, [M, D]);
}

#[test]
fn substractive_notation() {
    use roman_numbers::RomanDigit::*;

    assert_eq!(RomanNumber::from(4).0, [I, V]);
    assert_eq!(RomanNumber::from(44).0, [X, L, I, V]);
    assert_eq!(RomanNumber::from(3446).0, [M, M, M, C, D, X, L, V, I]);
    assert_eq!(RomanNumber::from(9).0, [I, X]);
    assert_eq!(RomanNumber::from(94).0, [X, C, I, V]);
}
