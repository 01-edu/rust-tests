use format_me::*;

#[test]
fn test_park() {
    let park = Park {
        name: Some("Central Park".to_owned()),
        park_type: ParkType::Garden,
        address: Some("Av. Sidónio Pais 4".to_owned()),
        cap: Some("1050-214".to_owned()),
        state: Some("Portugal".to_owned()),
    };

    assert_eq!(
        park.to_string(),
        "garden - Central Park, Av. Sidónio Pais 4, 1050-214 - Portugal"
    );
}

#[test]
fn test_empty_name() {
    let park = Park {
        name: None,
        park_type: ParkType::Forest,
        address: Some("Av. Sidónio Pais 4".to_owned()),
        cap: Some("1050-214".to_owned()),
        state: Some("Portugal".to_owned()),
    };

    assert_eq!(
        park.to_string(),
        "forest - No name, Av. Sidónio Pais 4, 1050-214 - Portugal"
    );
}

#[test]
fn test_empty_all() {
    let park = Park {
        name: None,
        park_type: ParkType::Playground,
        address: None,
        cap: None,
        state: None,
    };

    assert_eq!(
        park.to_string(),
        "playground - No name, No address, No cap - No state"
    );
}
