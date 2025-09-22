use count_factorial_steps::*;

#[test]
fn count_factorial_steps_edge_cases() {
    assert_eq!(0, count_factorial_steps(0));
    assert_eq!(0, count_factorial_steps(1));
    assert_eq!(0, count_factorial_steps(123));
}

#[test]
fn count_factorial_steps_normal_cases() {
    assert_eq!(6, count_factorial_steps(720));
    assert_eq!(10, count_factorial_steps(3628800));
}
