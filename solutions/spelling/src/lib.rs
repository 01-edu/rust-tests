/*
## spelling

### Instructions

In this exercise a number between 0 and 1000000 will be generated.
Your purpose is to create the function `spell` that will spell the numbers generated.

So, if the program generates the number:

- 1 your function will return the string "one"
- 14 your function will return the string "fourteen".
- 96 your function will return the string "ninety-six"
- 100 your function will return the string "one hundred".
- 101 your function will return the string "one hundred one"
- 348 your function will return the string "one hundred twenty-three"
- 1002 your function will return the string "one thousand two".
- 1000000 your function will return the string "one million"

### Notions

- https://doc.rust-lang.org/book/ch18-00-patterns.html

### Expected functions

```rust
pub fn spell(n: u64) -> String {}
```

### Usage

Here is a program to test your function.

```rust
fn main() {
    println!("{}", spell(348));
    println!("{}", spell(9996));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
three hundred forty-eight
nine thousand nine hundred ninety-six
student@ubuntu:~/[[ROOT]]/test$
```
 */
//use rand::Rng;

pub fn spell(n: u64) -> String {
    match n {
        0..=99 => spells_below_100(n),
        100..=999 => spells_hundreds(n),
        _ => spells_bignum(n),
    }
}

pub fn spells_below_100(n: u64) -> String {
    match n {
        0 => "zero".to_string(),
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "fifteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineeen".to_string(),
        20 => "twenty".to_string(),
        30 => "thirty".to_string(),
        40 => "forty".to_string(),
        50 => "fifty".to_string(),
        60 => "sixty".to_string(),
        70 => "seventy".to_string(),
        80 => "eighty".to_string(),
        90 => "ninety".to_string(),
        _ => {
            let rem = n % 10;
            format!("{}-{}", spells_below_100(n - rem), spells_below_100(rem))
        }
    }
}

pub fn spells_hundreds(n: u64) -> String {
    let div = n / 100;
    let rem = n % 100;
    let mut enc_str = format!("{} hundred", spells_below_100(div));
    if rem != 0 {
        enc_str = format!("{} {}", enc_str, spells_below_100(rem));
    }
    enc_str
}

pub fn spells_bignum(n: u64) -> String {
    let mut enc_chunks: Vec<String> = vec![];
    let mut chunks: Vec<u64> = vec![0; 7];
    let mut m = n;
    for e in chunks.iter_mut() {
        let rem = m % 1_000;
        m = m / 1_000;
        *e += rem;
    }
    for (idx, chunk) in chunks.into_iter().enumerate() {
        let substr = match idx {
            0 => "",
            1 => "thousand",
            2 => "million",
            3 => "billion",
            4 => "trillion",
            5 => "quadrillion",
            _ => "quintillion",
        };
        if chunk != 0 {
            enc_chunks.push(format!("{} {}", spell(chunk), substr).trim().to_string());
        }
    }
    enc_chunks.reverse();
    enc_chunks.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(spell(0), String::from("zero"));
        assert_eq!(spell(1), String::from("one"));
        assert_eq!(spell(14), String::from("fourteen"));
        assert_eq!(spell(20), String::from("twenty"));
        assert_eq!(spell(22), String::from("twenty-two"));
        assert_eq!(spell(101), String::from("one hundred one"));
        assert_eq!(spell(120), String::from("one hundred twenty"));
        assert_eq!(spell(123), String::from("one hundred twenty-three"));
        assert_eq!(spell(1000), String::from("one thousand"));
        assert_eq!(spell(1055), String::from("one thousand fifty-five"));
        assert_eq!(
            spell(1234),
            String::from("one thousand two hundred thirty-four")
        );
        assert_eq!(
            spell(10123),
            String::from("ten thousand one hundred twenty-three")
        );
        assert_eq!(
            spell(910112),
            String::from("nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            spell(651123),
            String::from("six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(spell(810000), String::from("eight hundred ten thousand"));
        assert_eq!(spell(1000000), String::from("one million"));
    }
}
