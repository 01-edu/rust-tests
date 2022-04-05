/*
## iterators

### Instructions

The Collatz Conjecture or 3x+1 problem can be summarized as follows:

Take any positive integer `n`.

- If `n` is even, you will divide `n` by 2 to get `n / 2`.
- If `n` is odd, you will multiply `n` by 3 and add 1 to get `3n + 1`.

Repeat the process indefinitely. The conjecture states that no matter which number you start with, you will always reach 1 eventually.

But sometimes the number grow significantly before it reaches 1. This can lead to an integer overflow and makes the algorithm unsolvable within the range of a number in u64. You will not have to worry about that in this exercise.

Given a number `n`, return the number of steps required to reach 1.

Examples:

Starting with n = 16, the steps would be as follows:

0- 16
1- 8
2- 4
3- 2
4- 1

Resulting in 4 steps. So for input n = 16, the return value would be 4.

### Notions

- [Trait Iterator](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [Collatz Conjecture](https://pt.wikipedia.org/wiki/Conjectura_de_Collatz)

### Expected functions

```rust

struct Collatz {
    v: u64,
    }

impl Iterator for Collatz {}

pub fn collatz(n: u64) -> Option<u64> {}
```

### Usage

Here is a program to test your function.

```rust
use iterators::*;

fn main() {
    println!("{:?}", collatz(4));
    println!("{:?}", collatz(5));
    println!("{:?}", collatz(6));
    println!("{:?}", collatz(7));
    println!("{:?}", collatz(12));
}
```

And its output:

```console
$ cargo run
Some(2)
Some(5)
Some(8)
Some(16)
Some(9)
$
```

*/
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(aux: u64) -> Self {
        Collatz { v: aux }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v % 2 == 0 {
            self.v = self.v / 2;
        } else {
            self.v = 3 * self.v + 1;
        }
        Some(Collatz { v: self.v })
    }
}

pub fn collatz(n: u64) -> Option<u64> {
    let mut num = 0;

    let mut aux = Collatz::new(n);

    while aux.v != 1 {
        println!("{}", aux.v);
        num += 1;
        aux.next();
    }

    Some(num)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_seven() {
        let test_value = vec![1, 2, 3, 4, 5, 6, 7];
        let test_result = vec![0, 1, 7, 2, 5, 8, 16];

        for i in 0..test_value.len() {
            assert_eq!(
                test_result[i],
                collatz(test_value[i]).unwrap(),
                "collatz({})",
                test_value[i]
            );
        }
    }

    #[test]
    fn test_big_numbers() {
        let test_value = vec![54, 888, 4372, 9999];
        let test_result = vec![112, 72, 33, 91];

        for i in 0..test_value.len() {
            assert_eq!(
                test_result[i],
                collatz(test_value[i]).unwrap(),
                "collatz({})",
                test_value[i]
            );
        }
    }

    #[test]
    fn test_iterator() {
        let mut aux = Collatz::new(133);
        let mut num = 0;
        let sequence = vec![
            133, 400, 200, 100, 50, 25, 76, 38, 19, 58, 29, 88, 44, 22, 11, 34, 17, 52, 26, 13, 40,
            20, 10, 5, 16, 8, 4, 2, 1,
        ];

        while aux.v != 1 {
            assert_eq!(aux.v, sequence[num]);
            aux.next();
            num += 1;
        }
    }
}
