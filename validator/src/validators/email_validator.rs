use crate::prelude::*;
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;

pub trait EmailValidator: Validator<String> {}

pub struct Email {
    pub message: Option<Cow<'static, str>>,
}

impl Default for Email { fn default() -> Self { Email { message: None } } }

impl Email {
    pub fn with_message(mut self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

static EMAIL_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").expect("valid email regex")
});

impl Validator<String> for Email {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if EMAIL_RE.is_match(value) {
            Ok(())
        } else {
            let default_msg = "Invalid email format";
            let msg = self.message.as_deref().unwrap_or(default_msg);
            Err(ValidationError::new("email", msg.to_string()))
        }
    }
}

impl EmailValidator for Email {}