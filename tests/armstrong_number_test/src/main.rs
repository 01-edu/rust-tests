use armstrong_number::*;

fn main() {
    println!("{:?}", is_armstrong_number(0));
    println!("{:?}", is_armstrong_number(1));
    println!("{:?}", is_armstrong_number(153));
    println!("{:?}", is_armstrong_number(370));
    println!("{:?}", is_armstrong_number(371));
    println!("{:?}", is_armstrong_number(407));
    println!("{:?}", is_armstrong_number(400));
    println!("{:?}", is_armstrong_number(198));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subject_examples() {
        assert_eq!(is_armstrong_number(0), Some(0));
        assert_eq!(is_armstrong_number(1), Some(1));
        assert_eq!(is_armstrong_number(153), Some(153));
        assert_eq!(is_armstrong_number(370), Some(370));
        assert_eq!(is_armstrong_number(371), Some(371));
        assert_eq!(is_armstrong_number(407), Some(407));
        assert_eq!(is_armstrong_number(400), None);
        assert_eq!(is_armstrong_number(198), None);
    }

    #[test]
    fn test_valid_numbers() {
        assert_eq!(is_armstrong_number(2), Some(2));
        assert_eq!(is_armstrong_number(3), Some(3));
        assert_eq!(is_armstrong_number(4), Some(4));
        assert_eq!(is_armstrong_number(5), Some(5));
        assert_eq!(is_armstrong_number(6), Some(6));
        assert_eq!(is_armstrong_number(7), Some(7));
        assert_eq!(is_armstrong_number(8), Some(8));
        assert_eq!(is_armstrong_number(9), Some(9));
        assert_eq!(is_armstrong_number(1634), Some(1634));
        assert_eq!(is_armstrong_number(8208), Some(8208));
        assert_eq!(is_armstrong_number(9474), Some(9474));
        assert_eq!(is_armstrong_number(54748), Some(54748));
        assert_eq!(is_armstrong_number(92727), Some(92727));
        assert_eq!(is_armstrong_number(93084), Some(93084));
    }

    #[test]
    fn test_invalid_numbers() {
        assert_eq!(is_armstrong_number(11), None);
        assert_eq!(is_armstrong_number(98), None);
        assert_eq!(is_armstrong_number(100), None);
        assert_eq!(is_armstrong_number(90438), None);
        assert_eq!(is_armstrong_number(940), None);
        assert_eq!(is_armstrong_number(85), None);
    }
}
