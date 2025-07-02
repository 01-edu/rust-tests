use spelling::*;

#[test]
fn test_predefined() {
    assert_eq!(spell(0), String::from("zero"));
    assert_eq!(spell(1), String::from("one"));
    assert_eq!(spell(14), String::from("fourteen"));
    assert_eq!(spell(20), String::from("twenty"));
    assert_eq!(spell(22), String::from("twenty-two"));
    assert_eq!(spell(101), String::from("one hundred one"));
    assert_eq!(spell(120), String::from("one hundred twenty"));
    assert_eq!(spell(123), String::from("one hundred twenty-three"));
    assert_eq!(spell(1000), String::from("one thousand"));
    assert_eq!(spell(1055), String::from("one thousand fifty-five"));
    assert_eq!(
        spell(1234),
        String::from("one thousand two hundred thirty-four")
    );
    assert_eq!(
        spell(10123),
        String::from("ten thousand one hundred twenty-three")
    );
    assert_eq!(
        spell(910112),
        String::from("nine hundred ten thousand one hundred twelve")
    );
    assert_eq!(
        spell(651123),
        String::from("six hundred fifty-one thousand one hundred twenty-three")
    );

    assert_eq!(spell(810000), String::from("eight hundred ten thousand"));
    assert_eq!(spell(1000000), String::from("one million"));
}
