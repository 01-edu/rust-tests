#[cfg(test)]
mod tests {
    use adding_twice::*;

    #[test]
    fn test_zero_twice() {
        let z = twice(add_curry(0));
        assert_eq!(z(1902), 1902);
    }

    #[test]
    fn test_negative_twice() {
        let neg = twice(add_curry(-32));
        assert_eq!(neg(6), -58);
        assert_eq!(neg(10), -54);
        assert_eq!(neg(660), 596);
        assert_eq!(neg(1902), 1838);
        assert_eq!(neg(463), 399);
        assert_eq!(neg(400000000), 399999936);
    }

    #[test]
    fn test_add10_twice() {
        let value = twice(add_curry(10));
        assert_eq!(value(6), 26);
        assert_eq!(value(10), 30);
        assert_eq!(value(600), 620);
        assert_eq!(value(1000), 1020);
        assert_eq!(value(463), 483);
        assert_eq!(value(400000000), 400000020);
    }

    #[test]
    fn test_add20_twice() {
        let value = twice(add_curry(20));
        assert_eq!(value(6), 46);
        assert_eq!(value(10), 50);
        assert_eq!(value(600), 640);
        assert_eq!(value(1000), 1040);
        assert_eq!(value(463), 503);
        assert_eq!(value(400000000), 400000040);
    }

    #[test]
    fn test_add30_twice() {
        let value = twice(add_curry(30));
        assert_eq!(value(6), 66);
        assert_eq!(value(10), 70);
        assert_eq!(value(600), 660);
        assert_eq!(value(1000), 1060);
        assert_eq!(value(463), 523);
        assert_eq!(value(400000000), 400000060);
    }
}
