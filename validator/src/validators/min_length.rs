use crate::prelude::*;

pub trait MinLengthValidator: Validator<String> {}

pub struct MinLength {
    pub limit: u32,
}

impl MinLength {
    pub fn new(max: u32) -> Self { Self { limit: max } }
}

impl Default for MinLength { fn default() -> Self { MinLength { limit: u32::MAX } } }

impl Validator<String> for MinLength {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if (value.len() as u32) < self.limit {
            Err(ValidationError::MinLength(self.limit))
        } else {
            Ok(())
        }
    }
}

impl MinLengthValidator for MinLength {}