use crate::validators::error::ValidationError;

/// A simple validation trait.
///
/// Implementors validate a value of type `T` and either return `Ok(())`
/// when the value is valid, or a `ValidationError` describing why it isn't.
///
/// Example
/// ```
/// use validator::validators::validator::Validator;
/// use validator::validators::positive_number_validator::Positive;
///
/// let v = Positive;
/// assert!(v.validate(&10).is_ok());
/// assert!(v.validate(&0).is_err());
/// ```
pub trait Validator<T> {
    /// Validate the provided `value`.
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}