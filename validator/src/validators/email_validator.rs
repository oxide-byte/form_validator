use crate::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;

pub trait EmailValidator: Validator<String> {}

pub struct Email;

impl Default for Email { fn default() -> Self { Email } }

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").expect("valid email regex")
});

impl Validator<String> for Email {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if EMAIL_RE.is_match(value) {
            Ok(())
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }
}

impl EmailValidator for Email {}