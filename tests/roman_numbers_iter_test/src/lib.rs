#[cfg(test)]
mod tests {
    use roman_numbers_iter::*;

    #[test]
    fn roman_numbers_iterator_test() {
        let numbers = [0, 8, 5, 13, 33, 49, 199, 499, 1532, 2348];

        for (n, e) in numbers.map(|n| (n, n + 1)) {
            assert_eq!(
                RomanNumber::from(n).next().unwrap().0,
                RomanNumber::from(e).0,
            );
        }
    }
}
