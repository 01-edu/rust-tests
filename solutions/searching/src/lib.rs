// In this exercise you will have to create your own `search` function.
// `search` receives an array of i32 and a key, then it will return the position
// of the given key in the array.

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let array = array.as_ref();

    if array.is_empty() {
        return None;
    }

    let mut left: usize = 0;
    let mut right: usize = array.len();

    while left <= right {
        let middle: usize = (left + right) / 2;

        let element = array.get(middle)?; // ? is used to fast error handling
        if key < *element {
            right = middle.checked_sub(1)?; // sub two num, checks underflow. If underflow, (None) is returned.
        } else if key > *element {
            left = middle + 1;
        } else {
            return Some(middle);
        }
    }

    return None;
}

// fn main() {
//     let ar = [1, 3, 4, 6, 8, 9, 11];
//     let f = search(&ar, 6);
//     println!(
//         "the element 6 is in the position {:?} in the array {:?}",
//         f, ar
//     );
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_a_value_in_an_array() {
        assert_eq!(search(&[6], 6), Some(0));
        assert_eq!(search(&[1, 2], 1), Some(0));
        assert_eq!(search(&[1, 2], 2), Some(1));
    }
    #[test]
    fn test_middle_of_an_array() {
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }

    #[test]
    fn test_beginning_of_an_array() {
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
    }

    #[test]
    fn test_end_of_an_array() {
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
    }

    #[test]
    fn test_long_array() {
        assert_eq!(
            search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
            Some(9)
        );
        assert_eq!(
            search(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
            Some(5)
        );
    }

    #[test]
    fn test_value_is_not_included() {
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 7), None);
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 0), None);
        assert_eq!(search(&[1, 3, 4, 6, 8, 9, 11], 13), None);
        assert_eq!(search(&[], 1), None);
    }
}
