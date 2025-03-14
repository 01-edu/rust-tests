pub use chrono::Utc;

#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    pub form_values: (&'static str, String),
    pub date: String,
    pub err: &'static str,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        Self {
            form_values: (field_name, field_value),
            date: Utc::now().format("%Y-%m-%d %H:%M:%S").to_string(),
            err,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    pub name: String,
    pub password: String,
}

impl Form {
    pub fn validate(&self) -> Result<(), FormError> {
        if self.name.is_empty() {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty",
            ));
        }

        let password_error =
            |m: &'static str| Err(FormError::new("password", self.password.clone(), m));

        if self.password.len() < 8 {
            return password_error("Password should be at least 8 characters long");
        } else if !(self.password.chars().any(|c| c.is_ascii_digit())
            && self.password.chars().any(|c| c.is_ascii_alphabetic())
            && self.password.chars().any(|c| c.is_ascii_punctuation()))
        {
            return password_error(
                "Password should be a combination of ASCII numbers, letters and symbols",
            );
        }

        Ok(())
    }
}
