use crate::prelude::*;

pub trait MaxLengthValidator: Validator<String> {}

pub struct MaxLength {
    pub limit: u32,
}

impl MaxLength {
    pub fn new(max: u32) -> Self { Self { limit: max } }
}

impl Default for MaxLength { fn default() -> Self { MaxLength { limit: u32::MAX } } }

impl Validator<String> for MaxLength {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if (value.len() as u32) > self.limit {
            Err(ValidationError::MaxLength(self.limit))
        } else {
            Ok(())
        }
    }
}

impl MaxLengthValidator for MaxLength {}