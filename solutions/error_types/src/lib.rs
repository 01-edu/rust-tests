pub use chrono::{NaiveDate, Utc};
// use chrono::format::ParseError;
use std::fmt;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (String, String),
    pub date: String,
    pub err: String,
}

impl FormError {
    pub fn new(field_name: String, field_value: String, err: String) -> FormError {
        FormError {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

impl fmt::Display for FormError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} - form values: {:?}, err: {}",
            &self.date, &self.form_values, &self.err
        )
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub first_name: String,
    pub last_name: String,
    pub birth: NaiveDate,
    pub birth_location: String,
    pub password: String,
}

impl Form {
    pub fn new(
        first_name: String,
        last_name: String,
        birth: NaiveDate,
        birth_location: String,
        password: String,
    ) -> Form {
        Form {
            first_name,
            last_name,
            birth,
            birth_location,
            password,
        }
    }

    // this will validate the form, it does not validate if
    // the requirements are not fulfilled
    pub fn validate(&self) -> Result<Vec<&str>, FormError> {
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

    fn user_name(&self) -> FormError {
        FormError::new(
            String::from("first_name"),
            self.first_name.clone(),
            String::from("No user name"),
        )
    }

    // all this is the part to validate all the parts of the form
    fn has_eight_char(&self) -> FormError {
        FormError::new(
            String::from("password"),
            self.password.clone(),
            String::from("At least 8 characters"),
        )
    }

    fn has_diff_ascii_char(&self) -> FormError {
        FormError::new(String::from("password"), self.password.clone(), String::from("Combination of different ASCII character types (numbers, letters and none alphanumeric characters)"))
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
