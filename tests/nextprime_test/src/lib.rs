#[cfg(test)]
mod tests {
    use nextprime::*;

    #[test]
    fn simple_tests() {
        assert_eq!(2, next_prime(0));
        assert_eq!(5, next_prime(5));
        assert_eq!(37, next_prime(32));
        assert_eq!(641, next_prime(633));
        assert_eq!(478157, next_prime(478152));
    }
}
