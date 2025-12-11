#[cfg(test)]
mod tests {
    use std::iter::zip;

    use iterators::*;

    #[test]
    fn test_first_seven() {
        let inputs = [1, 2, 3, 4, 5, 6, 7];
        let expected = [0, 1, 7, 2, 5, 8, 16];

        for (v, r) in zip(inputs, expected) {
            assert_eq!(collatz(v), Some(r));
        }
    }

    #[test]
    fn test_big_numbers() {
        let inputs = [54, 888, 4372, 9999];
        let expected = [112, 72, 33, 91];

        for (v, r) in zip(inputs, expected) {
            assert_eq!(collatz(v), Some(r));
        }
    }

    #[test]
    fn test_zero() {
        assert_eq!(collatz(0), None);
    }

    #[test]
    fn test_overflow() {
        let inputs = [
            6148914691236517207,
            6148914691236517209,
            6148914691236517211,
            6148914691236517213,
            6148914691236517215,
        ];

        for v in inputs {
            assert_eq!(collatz(v), None);
        }
    }
}
