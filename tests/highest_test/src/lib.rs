#[cfg(test)]
mod tests {
    use highest::*;

    #[test]
    fn test_inner() {
        let expected = [30, 50, 20, 70];
        let n = Numbers::new(expected);
        assert_eq!(n.inner(), &expected);
    }

    #[test]
    fn test_latest() {
        assert_eq!(Numbers::new([100, 0, 90, 30]).latest(), Some(30));
        assert_eq!(Numbers::new([]).latest(), None);
    }

    #[test]
    fn test_highest() {
        assert_eq!(Numbers::new([40, 100, 70]).highest(), Some(100));
        assert_eq!(Numbers::new([]).highest(), None);
    }

    #[test]
    fn test_highest_three() {
        assert_eq!(
            Numbers::new([10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]).highest_three(),
            Some([100, 90, 70])
        );
        assert_eq!(
            Numbers::new([40, 20, 40, 30]).highest_three(),
            Some([40, 40, 30])
        );
        assert_eq!(Numbers::new([30, 70]).highest_three(), None);
        assert_eq!(Numbers::new([40]).highest_three(), None);
        assert_eq!(Numbers::new([]).highest_three(), None);
        assert_eq!(
            Numbers::new([20, 10, 30]).highest_three(),
            Some([30, 20, 10])
        );
    }
}
