// How to avoid that students use sort without creating a program to
// restrict functions in the code??
// Just use a grep?
// Implement the insertion sort algorithm but implement it in a step
// by step so we can test that each step is corresponding with the algorithm

// Insertion sort algorithmAlgorithm

// To sort an array of size n in ascending order:

// 1: Iterate from arr[1] to arr[n] over the array.

// 2: Compare the current element (key) to its predecessor.

// 3: If the key element is smaller than its predecessor, compare it
// to the elements before. Move the greater elements one position up
// to make space for the swapped element.

pub fn insertion_sort<T: Ord + Clone>(ar: &mut [T], steps: usize) {
    assert!(steps > 0);
    for i in 1..steps + 1 {
        let key = ar[i].to_owned();
        let mut j: i32 = i as i32 - 1;
        while j >= 0 && key < ar[j as usize] {
            // move the value to the next position
            ar[(j + 1) as usize] = ar[j as usize].to_owned();
            j -= 1;
        }
        ar[(j + 1) as usize] = key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
