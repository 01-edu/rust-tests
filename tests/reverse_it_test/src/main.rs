use reverse_it::*;

fn main() {
    println!("{}", reverse_it(123456789));
    println!("{}", reverse_it(-123));
}

#[test]
fn reverse_it_test() {
    assert_eq!("321123", &reverse_it(123));
    assert_eq!("987654321123456789", &reverse_it(123456789));
    assert_eq!("00", &reverse_it(0));
    assert_eq!("-321123", &reverse_it(-123));
    assert_eq!("11", &reverse_it(1));
    assert_eq!("-84638474122147483648", &reverse_it(i32::MIN));
    assert_eq!("74638474122147483647", &reverse_it(i32::MAX));
}
