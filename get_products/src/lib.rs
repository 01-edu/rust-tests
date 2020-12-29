/*
## get_products

### Instructions

Create a function `get_products` that takes an array of integers, and returns an array of the products
of each index. For this exercise to be correct you will have to return the product of every index
except the current one.

### Example:

Input: arr[]  = {10, 3, 5, 6, 2}
Output: prod[]  = {180, 600, 360, 300, 900}

```rust
fn main() {
    let arr: Vec<usize> = vec![1, 7, 3, 4];
    let output = get_products(arr);
    println!("{:?}", output);
}
```
*/

pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut counter: usize = 0;
    let mut product_results: Vec<usize> = Vec::new();

    if arr.len() < 2 {
        return product_results;
    };

    for _ in arr.iter() {
        let mut prod: usize = 1;
        let mut others: Vec<usize> = arr.clone();
        others.remove(counter);
        for x in others.iter() {
            prod *= *x;
        }
        product_results.push(prod);
        counter += 1;
    }
    return product_results;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_multiple() {
        let arr: Vec<usize> = vec![1, 7, 3, 4];
        let output = get_products(arr);
        let arr2: Vec<usize> = vec![10, 3, 5, 6, 2];
        let output2 = get_products(arr2);
        assert_eq!(output, vec![84, 12, 28, 21]);
        assert_eq!(output2, vec![180, 600, 360, 300, 900]);
    }

    #[test]
    fn test_empty_case() {
        let arr: Vec<usize> = Vec::new();
        let output = get_products(arr);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_single_case() {
        let arr: Vec<usize> = vec![2];
        let output = get_products(arr);
        assert_eq!(output, vec![]);
    }
}
