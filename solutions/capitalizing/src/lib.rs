/*
## capitalizing

### Instructions

Complete the `capitalize_first` function that turns the first letter of a string uppercase.
Complete the `title_case` function that turns the first letter of each word in a string uppercase.
Complete the `change_case` function that turns the uppercase letters of a string into lowercase and
the lowercase letters into uppercase.

### Expected functions

```rust
fn capitalize_first(input: &str) -> String {}

fn title_case(input: &str) -> String {}

fn change_case(input: &str) -> String {}

```

### Usage

Here is a program to test your function.

```rust
fn main() {
    println!("{}", capitalize_first("joe is missing"));
    println!("{}", title_case("jill is leaving NOW"));
    println!("{}", change_case("heLLo THere"));
}
```

And its output

```console
student@ubuntu:~/[[ROOT]]/test$ cargo run
Joe is missing
Jill is leaving now
HEllO thERE
student@ubuntu:~/[[ROOT]]/test$
```
*/

pub fn capitalize_first(input: &str) -> String {
    input
        .chars()
        .take(1)
        .flat_map(char::to_uppercase)
        .chain(input.chars().skip(1))
        .collect::<String>()
}

pub fn title_case(input: &str) -> String {
    let mut res = String::with_capacity(input.len());
    let v: Vec<&str> = input.split(' ').collect();

    if input.is_empty() {
        return String::new();
    }

    for pat in v {
        let p = pat.to_lowercase();
        let slice: &str = &p[..];
        println!("-> {}", p);
        res += &capitalize_first(slice);
        res.push_str(" ");
    }
    let r = res.trim().to_string();
    return r;
}

pub fn change_case(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }

    let mut result = String::with_capacity(input.len());

    {
        for c in input.chars() {
            if c.is_uppercase() {
                let lower = c.to_lowercase().next().unwrap();
                result.push(lower);
            } else if c.is_lowercase() {
                let upper = c.to_uppercase().next().unwrap();
                result.push(upper);
            } else {
                result.push(c);
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("this is working"), "This is working");
    }

    #[test]
    fn test_titlle_case() {
        assert_eq!(title_case("this is a tittle"), "This Is A Tittle");
        assert_eq!(title_case("hello my name is carl"), "Hello My Name Is Carl");
    }

    #[test]
    fn test_change_case() {
        assert_eq!(change_case("PROgraming"), "proGRAMING");
        assert_eq!(change_case("heLLo THere"), "HEllO thERE");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
        assert_eq!(title_case(""), "");
        assert_eq!(change_case(""), "");
    }
}
