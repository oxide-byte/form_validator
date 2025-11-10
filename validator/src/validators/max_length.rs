use crate::prelude::*;
use std::borrow::Cow;

pub trait MaxLengthValidator: Validator<String> {}

pub struct MaxLength {
    pub limit: u32,
    pub message: Option<Cow<'static, str>>,
}

impl MaxLength {
    pub fn new(max: u32) -> Self { Self { limit: max, message: None } }
    pub fn with_message(mut self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

impl Default for MaxLength { fn default() -> Self { MaxLength { limit: u32::MAX, message: None } } }

impl Validator<String> for MaxLength {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if (value.len() as u32) > self.limit {
            let default_msg = format!("String too long (max {})", self.limit);
            let msg = self.message.as_deref().unwrap_or(&default_msg);
            Err(
                ValidationError::new("max_length", msg.to_string())
                    .with_param("limit", self.limit.to_string())
                    .with_param("len", value.len().to_string()),
            )
        } else {
            Ok(())
        }
    }
}

impl MaxLengthValidator for MaxLength {}