use vector_operations::*;

#[test]
fn test_addition() {
    let a = ThreeDVector { i: 3, j: 5, k: 2 };
    let b = ThreeDVector { i: 2, j: 7, k: 4 };
    let a_plus_b = ThreeDVector { i: 5, j: 12, k: 6 };
    assert_eq!(a + b, a_plus_b);
}

#[test]
fn test_subtraction() {
    let a = ThreeDVector { i: 3, j: 5, k: 2 };
    let b = ThreeDVector { i: 2, j: 7, k: 4 };
    let a_minus_b = ThreeDVector { i: 1, j: -2, k: -2 };
    assert_eq!(a - b, a_minus_b);
}
