use crate::validators::error::ValidationError;
use crate::validators::validator::Validator;

pub trait PositiveNumberValidator: Validator<i32> {}
pub struct Positive;

impl Default for Positive { fn default() -> Self { Positive } }

impl Validator<i32> for Positive {
    fn validate(&self, value: &i32) -> Result<(), ValidationError> {
        if *value > 0 {
            Ok(())
        } else {
            Err(ValidationError::MustBePositive)
        }
    }
}

impl PositiveNumberValidator for Positive {}