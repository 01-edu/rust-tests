/*
## error types

### Instructions

For this exercise you will have to implement an **error type**.

The main objective is to create a form validator, where you must implement a
error type. This must validate the password and the first name. The
first name must not be empty and the password must have at least 8 char and a combination of alphanumeric and none alphanumeric ASCII characters

ex: "asDd123=%" => good
    "asgfD"     => error
    "asdsdf2"   => error
    "sad_#$"    => error

Create a structure called `Form` that will have the following fields:

- `first_name`, that will be a string
- `last_name`, that will be a string
- `birth`, of type `NaiveDate` that will convert a string "2015-09-05" to a date of that format
- `sex`, SexType that must be a `enum` with the fields `Male` and `Female`
- `birth_location`, that will be a string
- `password`, that will be a string

You must also implement for this structure a function to initialize the structure, `new` and a function called
`validate` that will validate the form

For the error type you must create a type struct called `FErr`, that will be the type for the error
It must have the fields:

- `form_values`, this will be a tuple of strings that will save the value that the user inserted into the form

ex: ("password", "asdaSD_")
    ("first_name", "someone")

- `date`, that will have the date that the error occurred in the format "2020-12-14 09:33:41"
- `err`, that will have the error description:
    - "No user name"
    - "At least 8 characters"
    - "Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"

### Example

```rust
fn main() {
    let mut form_output = Form::new(
        String::from("Lee"),
        String::from("Silva"),
        create_date("2015-09-05"),
        SexType::Male,
        String::from("Africa"),
        String::from("qwqwsa1dty_"));

    println!("{:?}", form_output);
    // output:
    // Form { first_name: "Lee", last_name: "Silva", birth: 2015-09-05, sex: Male), birth_location: "Africa"), password: "qwqwsa1dty_"}
    println!("{:?}", form_output.validate().unwrap());
    // output:
    // ["Valid password", "Valid first name"]
    form_output.first_name = String::from("");
    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("first_name", ""), date: "2020-12-11 18:37:08", err: "No user name" }

    form_output.password = String::from("dty_1");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "dty_1"), date: "2020-12-11 18:49:31", err: "At least 8 characters" }

    form_output.password = String::from("asdasASd(_");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "asdasASd(_"), date: "2020-12-11 18:51:10", err: "Combination of different ASCII character types (numbers, letters and am none alphanumeric characters)" }

    form_output.password = String::from("asdasASd123SA");

    println!("{:?}", form_output.validate().unwrap_err());
    // output:
    // FErr { form_values: ("password", "asdasASd123SA", date: "2020-12-11 18:51:10", err: "Combination of different ASCII character types (numbers, letters and am none alphanumeric characters)" }
}
```

### Notions

- https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
- https://docs.rs/chrono/0.4.19/chrono/naive/struct.NaiveDate.html

*/
pub use chrono::{NaiveDate, Utc};
// use chrono::format::ParseError;
use std::fmt;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FErr {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FErr {
    pub fn new(name: String, error: String, err: String) -> FErr {
        FErr {
            form_values: (name, error),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

impl fmt::Display for FErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - form values: {:?}, err: {}",
            &self.date, &self.form_values, &self.err
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

// first name -> must be mandatory
// password: must be mandatory
//   - at least eight characters
//   - combination of all ASCII character types
//   - do not use sequests of characters
#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub fav_colour: Color,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        fav_colour: Color,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            fav_colour,
            birth_location,
            password,
        }
    }

    // this will validate the form, it does not validate if
    // the requirements are not fulfilled
    pub fn validate(&self) -> Result<Vec<&str>, FErr> {
        let v: Vec<&str> = vec![
            match &self.first_name {
                x if x == &String::from("") => Err(self.user_name()),
                _ => Ok("Valid first name"),
            }?,
            match &self.password {
                x if x.len() < 8 => Err(self.has_eight_char()),
                _ if !self.validate_ascii_char() => Err(self.has_diff_ascii_char()),
                _ => Ok("Valid password"),
            }?,
        ];
        Ok(v)
    }

    fn user_name(&self) -> FErr {
        FErr::new(
            String::from("first_name"),
            self.first_name.clone(),
            String::from("No user name"),
        )
    }

    // all this is the part to validate all the parts of the form
    fn has_eight_char(&self) -> FErr {
        FErr::new(
            String::from("password"),
            self.password.clone(),
            String::from("At least 8 characters"),
        )
    }

    fn has_diff_ascii_char(&self) -> FErr {
        FErr::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"))
    }

    fn validate_ascii_char(&self) -> bool {
        let chars: Vec<char> = self.password.chars().collect();
        chars.iter().any(|c| c.is_digit(10)) && chars.iter().any(|c| !c.is_alphanumeric())
    }
}

#[allow(dead_code)]
fn create_date(date: &str) -> NaiveDate {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_date(date: &str) -> NaiveDate {
        NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap()
    }

    #[derive(Debug)]
    struct TestForm<'a> {
        form: Form,
        validation: Result<Vec<&'a str>, FErr>,
    }

    impl<'a> TestForm<'_> {
        // all test cases
        fn new() -> Vec<TestForm<'a>> {
            vec![
                TestForm {
                    form : Form::new(
                    String::from("Katy"),
                    String::from("Silva"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Ok(vec!["Valid first name", "Valid password"]),
                },
                TestForm {
                    form : Form::new(
                    String::from(""),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Blue,
                    String::from("Africa"),
                    String::from("qwTw12&%$3sa1dty_")),
                    validation: Err(FErr {
                        form_values: (String::from("first_name"),
                        String::from("")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("No user name")}),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Green,
                    String::from("Africa"),
                    String::from("12345")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("12345")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("At least 8 characters") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Red,
                    String::from("Africa"),
                    String::from("sdASDsrW")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("sdASDsrW")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Blue,
                    String::from("Africa"),
                    String::from("dsGE1SAD213")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsGE1SAD213")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                },
                TestForm {
                    form : Form::new(
                    String::from("Someone"),
                    String::from("Bear"),
                    create_date("2015-09-05"),
                    Color::Green,
                    String::from("Africa"),
                    String::from("dsaSD&%DF!?=")),
                    validation: Err(FErr {
                        form_values: (String::from("password"), String::from("dsaSD&%DF!?=")),
                        date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
                        err: String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)") }),
                }
            ]
        }
    }

    #[test]
    fn test_error_type() {
        let form_cases = TestForm::new();

        for v in form_cases {
            assert_eq!(v.form.validate(), v.validation);
        }
    }
}
