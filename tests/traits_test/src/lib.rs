use traits::*;

#[test]
fn test_gives() {
    let apple = Fruit { weight_in_kg: 1.0 };
    assert_eq!(apple.gives(), 4.0);

    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 1.0,
    };
    assert_eq!(steak.gives(), 9.0);

    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 0.0,
    };
    assert_eq!(steak.gives(), 4.0);

    let steak = Meat {
        weight_in_kg: 1.5,
        fat_content: 0.3,
    };
    assert_eq!(steak.gives(), 8.25);
}

#[test]
fn test_eat() {
    let apple = Fruit { weight_in_kg: 1.0 };
    assert_eq!(apple.gives(), 4.0);

    let steak = Meat {
        weight_in_kg: 1.0,
        fat_content: 1.0,
    };

    let mut player1 = Player {
        name: "player1",
        strength: 1.0,
        score: 0,
        money: 0,
        weapons: vec!["knife"],
    };

    player1.eat(apple);
    assert_eq!(player1.strength, 5.0);

    player1.eat(steak);
    assert_eq!(player1.strength, 14.0);
}

#[test]
fn test_display() {
    let player1 = Player {
        name: "player1",
        strength: 1.0,
        score: 0,
        money: 0,
        weapons: vec!["knife", "shotgun"],
    };

    assert_eq!(
        player1.to_string(),
        "player1\nStrength: 1, Score: 0, Money: 0\nWeapons: [\"knife\", \"shotgun\"]"
    )
}
