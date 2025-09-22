use lucas_number::*;

#[test]
fn lucas_number_test() {
    assert_eq!(lucas_number(2), 3);
    assert_eq!(lucas_number(5), 11);
    assert_eq!(lucas_number(10), 123);
    assert_eq!(lucas_number(13), 521);
    assert_eq!(lucas_number(25), 167761);
}
