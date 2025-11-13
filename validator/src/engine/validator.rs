use crate::prelude::*;
#[cfg(feature = "async")]
use core::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;

/// A simple validation trait.
///
/// Implementors validate a value of type `T` and either return `Ok(())`
/// when the value is valid, or a `ValidationError` describing why it isn't.
///
/// Example
/// ```
/// use validator::prelude::Validator;
/// use validator::validators::positive_number_validator::Positive;
///
/// let v = Positive::default();
/// assert!(v.validate(&10).is_ok());
/// assert!(v.validate(&0).is_err());
/// ```
pub trait Validator<T> {
    /// Validate the provided `value`.
    fn validate(&self, value: &T) -> Result<(), ValidationError>;
}

/// An asynchronous validator variant.
///
/// By default, any synchronous `Validator<T>` is also an `AsyncValidator<T>`
/// via a blanket impl that wraps the synchronous call into a ready future.
#[cfg(feature = "async")]
pub trait AsyncValidator<T> {
    /// Asynchronously validate the provided `value`.
    fn validate_async<'a>(&'a self, value: &'a T) -> Pin<Box<dyn Future<Output = Result<(), ValidationError>> + 'a>>;
}

#[cfg(feature = "async")]
impl<V, T> AsyncValidator<T> for V
where
    V: Validator<T>,
{
    fn validate_async<'a>(&'a self, value: &'a T) -> Pin<Box<dyn Future<Output = Result<(), ValidationError>> + 'a>> {
        Box::pin(async move { self.validate(value) })
    }
}