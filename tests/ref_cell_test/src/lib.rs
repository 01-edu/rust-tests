use std::{cell::RefCell, rc::Rc};

use ref_cell::*;

#[test]
fn test_one() {
    let expected_messages = [
        "Info: This value would use 40% of your quota",
        "Warning: You have used up over 80% of your quota!",
        "Warning: You have used up over 100% of your quota!",
        "Error: You can't go over your quota!",
    ];

    let value = Rc::new(42);

    let track = Tracker::new(5);
    let _v = Rc::clone(&value);
    track.peek(&value); // 40%
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    track.set_value(&value); // 80%
    let _v = Rc::clone(&value);
    track.set_value(&value); // 100%
    let _v = Rc::clone(&value);
    track.set_value(&value); // >100%

    assert_eq!(track.messages.borrow().as_slice(), expected_messages);
}

#[test]
fn test_two() {
    let value = Rc::new(100);

    let track = Tracker::new(12);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);
    let _v = Rc::clone(&value);

    track.set_value(&value);

    let _v = Rc::clone(&value);

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 83% of your quota!"
    );

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 83% of your quota"
    );

    let _v = Rc::clone(&value);
    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 91% of your quota"
    );

    let _v = Rc::clone(&value);
    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Warning: You have used up over 100% of your quota!"
    );

    let _v = Rc::clone(&value);

    track.peek(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Info: This value would use 108% of your quota"
    );

    track.set_value(&value);
    assert_eq!(
        track.messages.borrow().last().unwrap(),
        "Error: You can't go over your quota!"
    );
}
