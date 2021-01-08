/*
## scores

### Instructions

Lets play a little!
Create a function `score` that given a string, computes the score for that given string.

Each letter has their value, you just have to sum the values of the letters in the
given string.

You'll need these:

| Letter                       | Value |
| ---------------------------- | :---: |
| A, E, I, O, U, L, N, R, S, T |   1   |
| D, G                         |   2   |
| B, C, M, P                   |   3   |
| F, H, V, W, Y                |   4   |
| K                            |   5   |
| J, X                         |   8   |
| Q, Z                         |  10   |

### Notions

- https://doc.rust-lang.org/book/ch18-00-patterns.html

### Expected functions

```rust
fn score(word: &str) -> u64 {}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    println!("{}", score("a"));
    println!("{}", score("ã ê Á?"));
    println!("{}", score("ThiS is A Test"));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
1
0
14
student@ubuntu:~/[[ROOT]]/test$
```
*/

pub fn score(word: &str) -> u64 {
    word.chars()
        .map(|x| match x.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0,
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("A"), 1);
        assert_eq!(score("h"), 4);
        assert_eq!(score("at"), 2);
        assert_eq!(score("Yes"), 6);
        assert_eq!(score("cellphones"), 17);
    }

    #[test]
    fn test_empty() {
        assert_eq!(score(""), 0);
        assert_eq!(score(" "), 0);
    }

    #[test]
    fn test_special() {
        assert_eq!(score("in Portugal NÃO stands for: no"), 30);
        assert_eq!(score("This is a test, comparação, têm Água?"), 36);
    }

    #[test]
    fn test_long() {
        assert_eq!(score("ThiS is A Test"), 14);
        assert_eq!(score("long sentences are working"), 34);
        assert_eq!(score("abcdefghijklmnopqrstuvwxyz"), 87);
    }
}
