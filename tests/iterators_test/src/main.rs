/*
## iterators

### Instructions

Create a method `new` that takes one number `usize` and initializes the `Number` struct.
This method will have to determinate if the given number is even or odd.

After that you will implement an iterator for the struct `Number` that returns a tuple (usize,usize,usize) containing each field of the struct Number.

The first position of the tuple will have the even numbers, the second will have the odd numbers, and the third will have the factorial numbers.

That being said, You will have to follow this rules:

- If the number is even you will have to calculate the factorial of the next odd number.
- If the number is odd you will have to calculate the factorial of the next even number.
- The tupple has to return all the numbers in the right positions.

### Notions

- https://doc.rust-lang.org/std/iter/trait.Iterator.html

*/
use iterators::*;

fn main() {
    let mut a = Number::new(4);
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
    println!("{:?}", a.next());
}

#[cfg(test)]
mod tests {
    use super::*;

    type Tuple = (usize, usize, usize);
    fn assert_eq_with_message(input: Number, actual: Tuple, expected: Tuple) {
        assert_eq!(
            actual, expected,
            "\n\t`{:?}`.next().unwrap() == `{:?}`, expected `{:?}`",
            input, actual, expected
        )
    }

    #[test]
    fn test_next() {
        let mut a = Number::new(4);
        assert_eq_with_message(a.clone(), a.next().unwrap(), (6, 5, 720));
        assert_eq_with_message(a.clone(), a.next().unwrap(), (6, 7, 5040));
        assert_eq_with_message(a.clone(), a.next().unwrap(), (8, 7, 40320));
        assert_eq_with_message(a.clone(), a.next().unwrap(), (8, 9, 362880));
        assert_eq_with_message(a.clone(), a.next().unwrap(), (10, 9, 3628800));
    }
}
