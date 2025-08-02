use minesweeper::*;

fn reset_solved(expected: &[&str]) -> Vec<String> {
    expected
        .iter()
        .map(|row| {
            row.chars()
                .map(|c| if c == '*' { c } else { ' ' })
                .collect()
        })
        .collect()
}

fn run_test(expected: &[&str]) {
    let to_solve = reset_solved(expected);
    let to_solve = to_solve.iter().map(|s| s.as_str()).collect::<Vec<_>>();

    assert_eq!(expected, solve_board(&to_solve))
}

#[test]
fn test_subject_example() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn test_no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn test_one_char() {
    #[rustfmt::skip]
    run_test(&[
        "*",
    ]);
}

#[test]
fn test_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn test_no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn test_no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn test_space_in_middle() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn test_complex_case() {
    #[rustfmt::skip]
    run_test(&[
        " 2**211",
        "13*43*1",
        "*334*32",
        "12**22*",
    ]);
}
