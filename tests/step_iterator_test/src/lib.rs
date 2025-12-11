#[cfg(test)]
mod tests {
    use step_iterator::*;

    #[test]
    fn test_next() {
        let mut step_iterator = StepIterator::new(0, 100, 10);
        assert_eq!(step_iterator.next(), Some(0));
        assert_eq!(step_iterator.next(), Some(10));
    }

    #[test]
    fn until_the_end() {
        for (i, v) in StepIterator::new(0, 100, 10).enumerate() {
            assert_eq!(i * 10, v);
        }
    }

    #[test]
    fn test_with_floats() {
        for (i, v) in StepIterator::new(0.0, 10.0, 0.5).enumerate() {
            assert_eq!(i as f64 * 0.5, v);
        }
    }

    #[test]
    fn test_with_floats_with_imperfect_range() {
        for (i, v) in StepIterator::new(0.3, 10.0, 0.5).enumerate() {
            assert_eq!(i as f64 * 0.5 + 0.3, v);
        }
    }
}
