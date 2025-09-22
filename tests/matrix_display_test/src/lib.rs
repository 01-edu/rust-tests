use matrix_display::*;

#[test]
fn it_works() {
    assert_eq!(
        Matrix::new(&[&[1, 2, 3], &[4, 5, 6], &[7, 8, 9]]).to_string(),
        "(1 2 3)\n(4 5 6)\n(7 8 9)"
    );
}

#[test]
fn test_matrix_col() {
    assert_eq!(
        Matrix::new(&[&[1], &[2], &[3]]).to_string(),
        "(1)\n(2)\n(3)"
    );
}

#[test]
fn test_matrix_row() {
    assert_eq!(Matrix::new(&[&[1, 2, 3]]).to_string(), "(1 2 3)");
}

#[test]
fn test_m_by_n_matrix() {
    assert_eq!(
        Matrix::new(&[&[1, 2, 3, 4, 5], &[6, 7, 8, 9, 10], &[11, 12, 13, 14, 15]]).to_string(),
        "(1 2 3 4 5)\n(6 7 8 9 10)\n(11 12 13 14 15)"
    );
}

#[test]
fn test_empty_matrix() {
    assert_eq!(Matrix::new(&[&[]]).to_string(), "()");
}
