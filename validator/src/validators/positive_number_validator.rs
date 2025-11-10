use crate::prelude::*;
use std::borrow::Cow;

pub trait PositiveNumberValidator: Validator<i32> {}

pub struct Positive {
    pub message: Option<Cow<'static, str>>,
}

impl Default for Positive { fn default() -> Self { Positive { message: None } } }

impl Positive {
    pub fn with_message(mut self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.message = Some(msg.into());
        self
    }
}

impl Validator<i32> for Positive {
    fn validate(&self, value: &i32) -> Result<(), ValidationError> {
        if *value > 0 {
            Ok(())
        } else {
            let default_msg = "Value must be a positive number";
            let msg = self.message.as_deref().unwrap_or(default_msg);
            Err(ValidationError::new("positive", msg.to_string()))
        }
    }
}

impl PositiveNumberValidator for Positive {}