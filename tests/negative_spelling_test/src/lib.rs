use negative_spelling::*;

#[test]
fn test_short_numbers() {
    assert_eq!(negative_spell(0), "zero");
    assert_eq!(negative_spell(-1), "minus one");
    assert_eq!(negative_spell(-14), "minus fourteen");
    assert_eq!(negative_spell(-20), "minus twenty");
    assert_eq!(negative_spell(-22), "minus twenty-two");
    assert_eq!(negative_spell(-101), "minus one hundred one");
    assert_eq!(negative_spell(-120), "minus one hundred twenty");
    assert_eq!(negative_spell(-123), "minus one hundred twenty-three");
}

#[test]
fn test_medium_numbers() {
    assert_eq!(negative_spell(-1000), "minus one thousand");
    assert_eq!(negative_spell(-1055), "minus one thousand fifty-five");
    assert_eq!(
        negative_spell(-1234),
        "minus one thousand two hundred thirty-four"
    );
    assert_eq!(
        negative_spell(-10123),
        "minus ten thousand one hundred twenty-three"
    );
}

#[test]
fn test_long_numbers() {
    assert_eq!(
        negative_spell(-910112),
        "minus nine hundred ten thousand one hundred twelve"
    );
    assert_eq!(
        negative_spell(-651123),
        "minus six hundred fifty-one thousand one hundred twenty-three"
    );

    assert_eq!(negative_spell(-810000), "minus eight hundred ten thousand");
    assert_eq!(negative_spell(-1000000), "minus one million");
}

#[test]
fn test_invalid_numbers() {
    assert_eq!(negative_spell(1), "error: positive number");
    assert_eq!(negative_spell(2390904), "error: positive number");
}
