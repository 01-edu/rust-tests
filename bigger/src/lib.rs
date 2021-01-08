/* ## bigger

### Instructions

Create the function `bigger` that gets the biggest positive number in the `HashMap`.

### Notions

- https://doc.rust-lang.org/stable/std/collections/struct.HashMap.html

### Expected functions

```rust
fn bigger(h: HashMap<&str, i32>) -> i32{}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    let mut hash = HashMap::new();
    hash.insert("Daniel", 122);
    hash.insert("Ashley", 333);
    hash.insert("Katie", 334);
    hash.insert("Robert", 14);

    println!(
        "The biggest of the elements in the HashMap is {}",
        bigger(hash)
    );
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
The biggest of the elements in the HashMap is 334
student@ubuntu:~/[[ROOT]]/test$
```
*/

use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32{

   let mut max_val = 0;
   if max_val >=0{
   for (_, v) in h.iter() {
       if *v > max_val {
           max_val = *v;
        }
    }
}
    return max_val

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive() {
        let mut f = HashMap::new();
    f.insert("Daniel", 12);
    f.insert("Ashley", 333);
    f.insert("Katie", 334);
    f.insert("Robert", 14);

        assert_eq!(334, bigger(f));
    }
    #[test]
    fn test_negative() {
        let mut f = HashMap::new();
    f.insert("Daniel", 41758712);
    f.insert("Ashley", 54551444);
    f.insert("Katie", 575556334);
    f.insert("Robert", 574148);

        assert_eq!(575556334, bigger(f));
    }
}
