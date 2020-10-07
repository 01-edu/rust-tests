/*
## banner

### Instructions

`Result` is a better version of the `Option` type that describes possible error instead
of possible absence

Create a structure called `Flag` that as the following elements:

  - short_hand: String
  - long_hand: String
  - desc: String

This structure must have associated to it a function called `opt_flag` that initializes the structure.
Receiving two references strings and returns the structure `Flag`. It should be used like this:

```rust
    let d = Flag::opt_flag("diff", "gives the difference between two numbers");

    println!("short hand: {}, long hand: {}, description: {}", d.short_hand, d.long_hand, d.desc);
    // output: "short hand: -d, long hand: --diff, description: gives the difference between two numbers"
```

It will be given a second structure called `FlagsHandler` that has just one element: `flags: HashMap<(String, String), Callback>`
And the following functions associated to it, for you to complete :

  - `add_flag`, that adds to the HashMap the flag and the Callback function.
  - `exec_func`, that executes the function using the flag provided and returns the result, that can
  be either a string with the value from the callback or an error.

It will also be provided a `type` called `Callback` being a function that is going to be used in the structure
and functions above. This function will be the callback for the flag associated to it.

You will have to create the following callback functions :

  - `div`, that converts the reference strings to `float`s and returns the `Result`, being the division of the `float`s
  or the standard (std) error: `ParseFloatError`.
  - `rem`, that converts the reference strings to `float`s and returns the `Result`, being the remainder of the division
  of the `float`s or the standard (std) error `ParseFloatError`.

### Notions

- https://doc.rust-lang.org/rust-by-example/error/result.html
- https://docs.rs/getopts/0.2.18/getopts/struct.Options.html#method.optflag

*/
use std::collections::HashMap;
use std::num::ParseFloatError;

#[allow(dead_code)]
fn div(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first_number = a.parse::<f32>()?;
    let second_number = b.parse::<f32>()?;
    Ok((first_number / second_number).to_string())
}
#[allow(dead_code)]
fn rem(a: &str, b: &str) -> Result<String, ParseFloatError> {
    let first_number = a.parse::<f32>()?;
    let second_number = b.parse::<f32>()?;
    Ok((first_number % second_number).to_string())
}

#[allow(dead_code)]
type Callback = fn(&str, &str) -> Result<String, ParseFloatError>;

#[allow(dead_code)]
struct FlagsHandler {
    flags: HashMap<(String, String), Callback>,
}
impl FlagsHandler {
    #[allow(dead_code)]
    fn add_flag(&mut self, flag: (String, String), func: Callback) {
        self.flags.insert(flag, func);
    }
    #[allow(dead_code)]
    fn exec_func(&mut self, flag: (String, String), argv: &[&str]) -> String {
        match self.flags[&flag](argv[0], argv[1]) {
            Ok(res) => res,
            Err(e) => e.to_string(),
        }
    }
}

#[derive(Debug)]
struct Flag {
    short_hand: String,
    long_hand: String,
    desc: String,
}
impl Flag {
    #[allow(dead_code)]
    fn opt_flag(l_h: &str, d: &str) -> Flag {
        let mut a = "-".to_string();
        a.push_str(&l_h[0..1]);
        Flag {
            short_hand: a,
            long_hand: "--".to_string() + &l_h.to_string(),
            desc: d.to_string(),
        }
    }
}

/* Example
fn main() {
    let mut handler = FlagsHandler { flags: HashMap::new() };

    let d = Flag::opt_flag("division", "divides the values, formula (a / b)");
    let r = Flag::opt_flag(
        "remainder",
        "remainder of the division between two values, formula (a % b)",
    );

    handler.add_flag((d.short_hand, d.long_hand), div);
    handler.add_flag((r.short_hand, r.long_hand), rem);

    // output: "0.5"
    println!("{:?}", handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]));

    // output: "0.0"
    println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]));
    
    // output: "ERROR: invalid float literal"
    println!("{:?}",handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]));
    
    // output: "ERROR: invalid float literal"
    println!("{:?}",handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "fd"]));
}
*/

#[cfg(test)]
mod tests {
    use super::*;

    fn init() -> FlagsHandler {
        let d = Flag::opt_flag("division", "divides two numbers");
        let r = Flag::opt_flag(
            "remainder",
            "gives the remainder of the division between two numbers",
        );
        let mut handler = FlagsHandler { flags: HashMap::new() };

        handler.add_flag((d.short_hand, d.long_hand), div);
        handler.add_flag((r.short_hand, r.long_hand), rem);
        return handler;
    }

    #[test]
    fn ok_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "2.0"]),
            "0.5"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "2.0"]),
            "0"
        );
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["12.323", "212.32"]),
            "0.05803975"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["12.323", "212.32"]),
            "12.323"
        );
    }

    #[test]
    fn error_test() {
        let mut handler = init();
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["a", "2.0"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "f"]),
            "invalid float literal"
        );
        assert_eq!(
            handler.exec_func(("-d".to_string(), "--division".to_string()), &["1.0", "0.0"]),
            "inf"
        );
        assert_eq!(
            handler.exec_func(("-r".to_string(), "--remainder".to_string()), &["2.0", "0.0"]),
            "NaN"
        );
    }
}

