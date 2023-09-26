/*
## binary_search

### Instructions

Write a function called `binary_search` that performs a binary search on a sorted list of integers. The function should take the following arguments:

```rust
pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
	// Your code goes here
}
```

- `sorted_list: &[i32]`: A reference to a sorted list of integers to search within.
- `target: i32`: The integer value to search for within the list.

The function should return an `Option<usize>` representing the index of the `target` in the list if found, or `None` if the `target` is not in the list.

### Usage

Here is a possible runner to test your function:

```rust
use binary_search::binary_search;

fn main() {
    let sorted_list = vec![1, 3, 5, 7, 9, 11, 13];
    let target = 7;
    
    match binary_search(&sorted_list, target) {
        Some(index) => println!("{} found at index {}", target, index),
        None => println!("{} not found in the list", target),
    }
}
```

And its output:

```console
$ cargo run
7 found at index 3
```

### Resources

-[Binary search alogrithm](https://en.wikipedia.org/wiki/Binary_search_algorithm)
*/

pub fn binary_search(sorted_list: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high;
    let len = sorted_list.len();
    
    if len == 0 {
        return None;
    } else {
        high = len - 1;
    }

    while low <= high {
        let mid = (low + high) / 2;

        if sorted_list[mid] == target {
            return Some(mid);
        }

        if sorted_list[mid] < target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    None
}