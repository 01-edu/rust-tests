use blood_types::*;

use std::{collections::HashMap, hash::Hash};

fn slices_eq_unordered<T: Eq + Hash>(a: &[T], b: &[T]) -> bool {
    let count_elems = |arr| {
        let mut map = HashMap::new();
        for item in arr {
            *map.entry(item).or_insert(0) += 1;
        }
        map
    };

    count_elems(a) == count_elems(b)
}

#[test]
fn test_unexistent_blood_type() {
    assert!("AO-".parse::<BloodType>().is_err());
}

#[test]
fn check_blood_type_relationships() {
    let relationships = [
        ("AB-", "A+", false),
        ("A-", "A+", false),
        ("AB-", "A-", true),
        ("AB-", "O+", false),
        ("AB+", "O+", true),
        ("AB-", "O-", true),
    ];

    relationships
        .into_iter()
        .map(|(t1, t2, e)| {
            (
                t1.parse::<BloodType>().unwrap(),
                t2.parse::<BloodType>().unwrap(),
                e,
            )
        })
        .for_each(|(t1, t2, e)| assert_eq!(t1.can_receive_from(t2), e));
}

#[test]
fn test_ab_plus_from_str() {
    assert!(matches!(
        "AB+".parse().unwrap(),
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive
        }
    ));
}

#[test]
fn test_a_negative_from_str() {
    assert!(matches!(
        "A-".parse().unwrap(),
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative
        }
    ));
}

#[test]
fn test_ab_plus_donors() {
    let donors = "AB+".parse::<BloodType>().unwrap().donors();
    let expected =
        ["AB-", "A-", "B-", "O-", "AB+", "A+", "B+", "O+"].map(|s| s.parse::<BloodType>().unwrap());
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_a_neg_donors() {
    let donors = "A-".parse::<BloodType>().unwrap().donors();
    let expected = ["A-", "O-"].map(|s| s.parse::<BloodType>().unwrap());
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_o_neg_donors() {
    let donors = "O-".parse::<BloodType>().unwrap().donors();
    let expected = ["O-"].map(|s| s.parse::<BloodType>().unwrap());
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_ab_pos_recipients() {
    let recipients = "AB+".parse::<BloodType>().unwrap().recipients();
    let expected = ["AB+"].map(|s| s.parse::<BloodType>().unwrap());
    assert!(slices_eq_unordered(&recipients, &expected));
}

#[test]
fn test_a_neg_recipients() {
    let recipients = "A-".parse::<BloodType>().unwrap().recipients();
    let expected = ["A-", "AB+", "A+", "AB-"].map(|s| s.parse::<BloodType>().unwrap());
    assert!(slices_eq_unordered(&recipients, &expected));
}

#[test]
fn test_debug_impl() {
    assert_eq!(
        format!(
            "{:?}",
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            }
        ),
        "AB+"
    );

    assert_eq!(
        format!(
            "{:?}",
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            }
        ),
        "A-"
    );
}
