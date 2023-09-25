use binary_search::*;

fn main() {
    println!("{:?}", binary_search(&[1, 2, 3, 4, 5], 3));
    println!("{:?}", binary_search(&[1, 2, 3, 4, 5], 42));
    println!("{:?}", binary_search(&[], 42));
}


#[cfg(test)]
mod tests {
    use binary_search::*;

    #[test]
    fn simple_sorted_list() {
        let sorted = [1, 2, 3, 4, 5, 6];
        let search_target = 4;
        let expected_index = Some(3);
        assert!(binary_search(&sorted, search_target) == expected_index);
    }

    #[test]
    fn target_not_in_list() {
        let sorted = [1, 2, 3, 4, 5, 6];
        let search_target = 1337;
        let expected_index = None;
        assert!(binary_search(&sorted, search_target) == expected_index);
    }

    #[test]
    fn empty_list() {
        let sorted = [];
        let search_target = 42;
        let expected_index = None;
        assert!(binary_search(&sorted, search_target) == expected_index);
    }
}
