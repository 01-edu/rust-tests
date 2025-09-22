use blood_types_s::*;
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
fn check_blood_type_relationships() {
    let relationships = [
        (
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            false,
        ),
        (
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Positive,
            },
            false,
        ),
        (
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::A,
                rh_factor: RhFactor::Negative,
            },
            true,
        ),
        (
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            false,
        ),
        (
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Positive,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Positive,
            },
            true,
        ),
        (
            BloodType {
                antigen: Antigen::AB,
                rh_factor: RhFactor::Negative,
            },
            BloodType {
                antigen: Antigen::O,
                rh_factor: RhFactor::Negative,
            },
            true,
        ),
    ];

    relationships
        .into_iter()
        .for_each(|(t1, t2, e)| assert_eq!(t1.can_receive_from(t2), e));
}

#[test]
fn test_ab_pos_donors() {
    let donors = BloodType {
        antigen: Antigen::AB,
        rh_factor: RhFactor::Positive,
    }
    .donors();
    let expected = [
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::B,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Positive,
        },
    ];
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_a_neg_donors() {
    let donors = BloodType {
        antigen: Antigen::A,
        rh_factor: RhFactor::Negative,
    }
    .donors();
    let expected = [
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::O,
            rh_factor: RhFactor::Negative,
        },
    ];
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_o_neg_donors() {
    let donors = BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Negative,
    }
    .donors();
    let expected = [BloodType {
        antigen: Antigen::O,
        rh_factor: RhFactor::Negative,
    }];
    assert!(slices_eq_unordered(&donors, &expected));
}

#[test]
fn test_ab_pos_recipients() {
    let recipients = BloodType {
        antigen: Antigen::AB,
        rh_factor: RhFactor::Positive,
    }
    .recipients();
    let expected = [BloodType {
        antigen: Antigen::AB,
        rh_factor: RhFactor::Positive,
    }];
    assert!(slices_eq_unordered(&recipients, &expected));
}

#[test]
fn test_a_neg_recipients() {
    let recipients = BloodType {
        antigen: Antigen::A,
        rh_factor: RhFactor::Negative,
    }
    .recipients();
    let expected = [
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::A,
            rh_factor: RhFactor::Positive,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Negative,
        },
        BloodType {
            antigen: Antigen::AB,
            rh_factor: RhFactor::Positive,
        },
    ];
    assert!(slices_eq_unordered(&recipients, &expected));
}
