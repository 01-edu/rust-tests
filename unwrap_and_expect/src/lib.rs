/*
## unwrap

### Instructions

It will be given a function called **odd_to_even**, that returns an `Result`. If its an error it will
return a tuple with a string, indicating the error, and a vector with the elements that justifies the error

The objective is to execute the `odd_to_even` function and handle the error given by it

Create the following functions that receives a vector :

  - `expect` that returns the error adding the sting "ERROR "
  - `unwrap_or` that in case of error returns an empty vector
  - `unwrap_err` that returns error if its `Ok` and returns the
     string containing the error in case of `Err`
  - `unwrap` that unwraps the `Result`
  - `unwrap_or_else` that in case of error returns a the 
    vector that justifies the error

### Example

```rust
fn main() {
    // println!("{:?}", expect(vec![1, 3, 2, 5]));
    println!("{:?}", unwrap_or(vec![1, 3, 2, 5]));
    // output : []
    println!("{:?}", unwrap_or(vec![1, 3, 5]));
    // output : [2, 4, 6]
    println!("{:?}", unwrap_err(vec![1, 3, 2, 5]));
    // output : ("There is a even value in the vector!", [2])
    // println!("{:?}", unwrap_err(vec![1, 3, 5]));
    println!("{:?}", unwrap(vec![1, 3, 5]));
    // output : [2, 4, 6]
    // println!("{:?}", unwrap(vec![1, 3, 2, 5]));
    println!("{:?}", unwrap_or_else(vec![1, 3, 5]));
    // output : [2, 4, 6]
    println!("{:?}", unwrap_or_else(vec![3, 2, 6, 5]));
    // output : [2, 6]
}
```
### Notions

- https://doc.rust-lang.org/std/?search=unwrap
*/

pub fn odd_to_even(data: Vec<u32>) -> Result<Vec<u32>, (String, Vec<u32>)> {
    let mut a = Vec::new();
    a.extend(data.iter().filter(|&value| value % 2 == 0));
    if a.len() != 0 {
        return Err(("There is a even value in the vector!".to_string(), a));
    }
    a.extend(data.iter().map(|&value| {
        value + 1
    }));
    Ok(a)
}

pub fn expect(v: Vec<u32>) -> Vec<u32> {
      odd_to_even(v).expect("ERROR ")
}

pub fn unwrap_or(v: Vec<u32>) -> Vec<u32> {
        odd_to_even(v).unwrap_or(vec![])
}

pub fn unwrap_err(v: Vec<u32>) -> (String, Vec<u32>) {
        odd_to_even(v).unwrap_err()
}

pub fn unwrap(v: Vec<u32>) -> Vec<u32> {
        odd_to_even(v).unwrap()
}

pub fn unwrap_or_else(v: Vec<u32>) -> Vec<u32> {
        odd_to_even(v).unwrap_or_else(|(_, x)| x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "ERROR : (\"There is a even value in the vector!\", [2])")]
    fn test_expect() {
        expect(vec![1, 3, 2, 5]);
    }
    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: (\"There is a even value in the vector!\", [2])")]
    fn test_unwrap() {
        unwrap(vec![1, 3, 2, 5]);
    }
    #[test]
    #[should_panic]
    fn test_unwrap_err() {
        unwrap_err(vec![1, 3, 5]);
    }
    #[test]
    fn test_unwrap_or() {
        assert_eq!(unwrap_or(vec![1, 3, 2, 5]), vec![]);
    }
    #[test]
    fn test_unwrap_or_else() {
        assert_eq!(unwrap_or_else(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap_or_else(vec![6, 8, 3, 2, 5, 4]), vec![6, 8, 2, 4]);
    }
    #[test]
    fn test_ok() {
        assert_eq!(expect(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap_or(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap_or_else(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap(vec![1, 3, 5]), vec![2, 4, 6]);
        assert_eq!(unwrap_err(vec![1, 2, 3, 4, 5]).0, "There is a even value in the vector!");
        assert_eq!(unwrap_err(vec![1, 2, 3, 4, 5]).1, vec![2, 4]);
    }
}
