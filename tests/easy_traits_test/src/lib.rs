use easy_traits::*;

#[test]
fn test_append_str() {
    assert_eq!("hello there!", "hello".to_owned().append_str(" there!"));

    assert_eq!(
        "hello, how are you?",
        "hello".to_owned().append_str(", how are you?")
    );

    assert_eq!("hello!", "hello!".to_owned().append_str(""));

    assert_eq!("hello! ", "hello!".to_owned().append_str(" "));

    assert_eq!(
        "h!e!l?lo. the,.re!",
        String::new().append_str("h!e!l?lo. the,.re!")
    );
}

#[test]
fn test_remove_punctuation() {
    assert_eq!("", "!?.,!?.,".to_owned().remove_punctuation_marks());

    assert_eq!(
        "hello there",
        "hello there".to_owned().remove_punctuation_marks()
    );

    assert_eq!(
        "hello there how are you",
        ".hello there, how are you?!"
            .to_owned()
            .remove_punctuation_marks()
    );

    assert_eq!("-1555", "-1555".to_owned().remove_punctuation_marks());
}

#[test]
fn test_append_number() {
    assert_eq!("-1", String::new().append_number(-1.));

    assert_eq!("-1-5", "-1".to_owned().append_number(-5.));

    assert_eq!("405.5", "40".to_owned().append_number(5.5));
}
