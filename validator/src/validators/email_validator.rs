use crate::prelude::*;
use regex::Regex;

pub trait EmailValidator: Validator<String> {}

pub struct Email;

impl Default for Email { fn default() -> Self { Email } }

impl Validator<String> for Email {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
        if re.is_match(value) {
            Ok(())
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
}

impl EmailValidator for Email {}