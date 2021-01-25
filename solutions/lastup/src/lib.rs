/*
## lastup

### Instructions

Complete the `lastup` function that takes a string and puts the last letter of each word in uppercase and the rest in lowercase.

### Expected Functions

```rust
pub fn lastup(input: &str) -> String {
}
```

### Usage

Here is a program to test your function.

```rust
use lastup::lastup;

fn main() {
    println!("{}", lastup("joe is missing"));
    println!("{}", lastup("jill is leaving a"));
    println!("{}", lastup("HOW ARE YOU TODAY?"));
}

```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
joE iS missinG
jilL iS leavinG A
hellO therE
student@ubuntu:~/[[ROOT]]/test$
```
 */

pub fn cap(i: &str) -> String {
    let mut v: Vec<char> = i.chars().rev().collect();
    if v[0].is_alphabetic() {
        v[0] = v[0].to_uppercase().nth(0).unwrap();
    } else {
        v[1] = v[1].to_uppercase().nth(0).unwrap();
    }
    let res = v.into_iter().rev().collect();
    return res;
}

pub fn lastup(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    let v: Vec<&str> = input.split(' ').collect();

    if input.is_empty() {
        return String::new();
    }

    for pat in v {
        let p = pat.to_lowercase();
        let slice: &str = &p[..];
        res += &cap(slice);
        res.push_str(" ");
    }
    let r = res.trim().to_string();
    return r;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lastup() {
        assert_eq!(lastup("hello"), "hellO");
        assert_eq!(lastup("this is working"), "thiS iS workinG");
        assert_eq!(lastup("HOW ARE YOU TODAY?"), "hoW arE yoU todaY");
        assert_eq!(lastup("tHIs IS wOrking 10"), "thiS iS workinG 10");
    }

    #[test]
    fn test_empty() {
        assert_eq!(lastup(""), "");
    }
}
