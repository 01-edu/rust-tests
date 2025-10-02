#[cfg(test)]
mod tests {
    use insertion_sort::*;

    #[test]
    fn it_works() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        let len = target.len();
        insertion_sort(&mut target, len - 1);
        assert_eq!(&[1, 2, 3, 4, 5, 6, 7, 8], &target);
    }

    #[test]
    fn test_first_step() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        insertion_sort(&mut target, 1);
        assert_eq!(&[3, 5, 7, 2, 1, 6, 8, 4], &target);
    }

    #[test]
    fn test_second_step() {
        let mut target = [5, 3, 7, 2, 1, 6, 8, 4];
        insertion_sort(&mut target, 2);
        assert_eq!(&[3, 5, 7, 2, 1, 6, 8, 4], &target);
    }
}
