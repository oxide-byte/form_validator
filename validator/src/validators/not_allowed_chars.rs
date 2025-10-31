use crate::prelude::*;

pub trait NotAllowedCharsValidator: Validator<String> {}
pub struct NotAllowedChars;

impl Default for NotAllowedChars { fn default() -> Self { NotAllowedChars } }

impl Validator<String> for NotAllowedChars {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if !value.contains("Z") {
            Ok(())
        } else {
            Err(ValidationError::NotAllowedChars("Z".to_string()))
        }
    }
}

impl NotAllowedCharsValidator for NotAllowedChars {}