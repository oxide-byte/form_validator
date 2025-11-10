use crate::prelude::*;
use std::borrow::Cow;

pub trait NotAllowedCharsValidator: Validator<String> {}
pub struct NotAllowedChars {
    forbidden: Vec<String>,
    pub message: Option<Cow<'static, str>>,
}

impl NotAllowedChars {
    pub fn new<I, S>(forbidden: I) -> Self
    where
        I: IntoIterator<Item=S>,
        S: Into<String>,
    {
        Self { forbidden: forbidden.into_iter().map(Into::into).collect(), message: None }
    }

    pub fn with_message(mut self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

impl Default for NotAllowedChars { fn default() -> Self { NotAllowedChars { forbidden: Vec::default(), message: None } } }

impl Validator<String> for NotAllowedChars {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        for c in self.forbidden.iter() {
            if !value.contains(c) {} else {
                let default_msg = "Value contains not allowed string";
                let msg = self.message.as_deref().unwrap_or(default_msg);
                return Err(
                    ValidationError::new("not_allowed_chars", msg.to_string())
                        .with_param("hit", c.clone())
                );
            }
        }
        Ok(())
    }
}

impl NotAllowedCharsValidator for NotAllowedChars {}