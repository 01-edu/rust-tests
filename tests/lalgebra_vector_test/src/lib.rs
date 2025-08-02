use lalgebra_vector::*;

#[test]
fn dot_product() {
    assert_eq!(Vector(vec![1, 3, -5]).dot(Vector(vec![4, -2, -1])), Some(3));

    assert_eq!(Vector(vec![1, 3, -5]).dot(Vector(vec![4, -2])), None);
}

#[test]
fn addition() {
    assert_eq!(
        Vector(vec![1, 3, -5]) + Vector(vec![4, -2, -1]),
        Some(Vector(vec![5, 1, -6]))
    );

    assert_eq!(Vector(vec![1, 3, -5]) + Vector(vec![2, 4, -2, -1]), None);
}
