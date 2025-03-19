use convert_case::{Case, Casing};
use edit_distance::edit_distance;

pub fn expected_variable(compare: &str, expected: &str) -> Option<String> {
    let compare = compare.to_lowercase();
    let expected = expected.to_lowercase();

    if !compare.is_case(Case::Camel) && !compare.is_case(Case::Snake) {
        return None;
    }

    let distance = edit_distance(&compare, &expected);

    let percentage = 100 - (distance * 100 / expected.len()).min(100);
    if percentage >= 50 {
        Some(format!("{percentage}%"))
    } else {
        None
    }
}
