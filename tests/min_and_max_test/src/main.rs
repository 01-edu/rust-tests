use min_and_max::*;

fn main() {
    let nb_1 = 9;
    let nb_2 = 4;
    let nb_3 = 2;
    let (min, max) = min_and_max(nb_1, nb_2, nb_3);
    println!("The minimum is {}, the maximum is {}", min, max);
}

#[test]
fn test_min_and_max() {
    let (min, max) = min_and_max(0, 0, 0);

    assert_eq!(min, 0);
    assert_eq!(max, 0);

    let (min, max) = min_and_max(1, 2, 3);

    assert_eq!(min, 1);
    assert_eq!(max, 3);

    let (min, max) = min_and_max(-1, -2, -3);

    assert_eq!(min, -3);
    assert_eq!(max, -1);

    let (min, max) = min_and_max(14, -12, 3);

    assert_eq!(min, -12);
    assert_eq!(max, 14);
}
