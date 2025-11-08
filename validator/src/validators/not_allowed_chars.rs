use crate::prelude::*;

pub trait NotAllowedCharsValidator: Validator<String> {}
pub struct NotAllowedChars {
    forbidden: Vec<String>,
}

impl NotAllowedChars {
    pub fn new<I, S>(forbidden: I) -> Self
    where
        I: IntoIterator<Item=S>,
        S: Into<String>,
    {
        Self { forbidden: forbidden.into_iter().map(Into::into).collect() }
    }
}

impl Default for NotAllowedChars { fn default() -> Self { NotAllowedChars { forbidden: Vec::default() } } }

impl Validator<String> for NotAllowedChars {
    fn validate(&self, value: &String) -> Result<(), ValidationError> {
        for c in self.forbidden.iter() {
            if !value.contains(c) {} else {
                return Err(ValidationError::NotAllowedChars(c.clone()));
            }
        }
        Ok(())
    }
}

impl NotAllowedCharsValidator for NotAllowedChars {}