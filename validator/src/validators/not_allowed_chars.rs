use crate::validators::error::ValidationError;
use crate::validators::validator::Validator;

pub struct NotAllowedChars;

impl Validator<String> for NotAllowedChars {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        if !value.contains("Z") {
            Ok(())
        } else {
            Err(ValidationError::NotAllowedChars("Z".to_string()))
        }
    }
}
