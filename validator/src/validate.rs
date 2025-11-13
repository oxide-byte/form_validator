use crate::engine::error::ValidationError;
use crate::engine::validator::Validator;
use core::marker::PhantomData;
use std::collections::HashMap;
#[cfg(feature = "async")]
use std::future::Future;
#[cfg(feature = "async")]
use std::pin::Pin;
#[cfg(feature = "async")]
use crate::engine::validator::AsyncValidator;

/// Trait for types that can validate themselves.
///
/// Implementations should return `Ok(())` when valid or a `ValidationError`
/// describing the first encountered problem.
pub trait Validate {
    /// Validate and stop at the first encountered error (default behavior).
    fn validate(&self) -> Result<(), ValidationError>;

    /// Validate all rules and collect all errors per field.
    /// Returns Ok(()) when there are no errors, or Err(HashMap) where keys
    /// identify the field (or index for tuple structs) and values are vectors of
    /// corresponding ValidationError for that field.
    fn complete_validate(&self) -> Result<(), HashMap<String, Vec<ValidationError>>> {
        // Default implementation falls back to short-circuit validation.
        match self.validate() {
            Ok(()) => Ok(()),
            Err(e) => {
                let mut map: HashMap<String, Vec<ValidationError>> = HashMap::new();
                // Without field context, use a generic key and collect single error.
                map.insert("value".to_string(), vec![e]);
                Err(map)
            }
        }
    }
}

/// Async variant of [`Validate`].
#[cfg(feature = "async")]
pub trait ValidateAsync {
    /// Validate and stop at the first encountered error (async).
    fn validate_async(&self) -> Pin<Box<dyn Future<Output = Result<(), ValidationError>> + '_>>;

    /// Validate all rules and collect all errors per field (async).
    fn complete_validate_async(
        &self,
    ) -> Pin<
        Box<
            dyn Future<
                Output = Result<(), HashMap<String, Vec<ValidationError>>>,
            > + '_,
        >,
    > {
        // Default async implementation mirrors the sync fallback.
        Box::pin(async move {
            match self.validate_async().await {
                Ok(()) => Ok(()),
                Err(e) => {
                    let mut map: HashMap<String, Vec<ValidationError>> = HashMap::new();
                    map.insert("value".to_string(), vec![e]);
                    Err(map)
                }
            }
        })
    }
}

/// Adapter that pairs a value `T` with a validator `V` (which implements
/// `Validator<T>`) and provides a `Validate` implementation.
///
/// This is handy to make existing fields validate-able without introducing
/// newtype wrappers.
pub struct With<V, T>(pub T, PhantomData<V>);

impl<V, T> With<V, T> {
    pub fn new(value: T) -> Self { Self(value, PhantomData) }
    pub fn into_inner(self) -> T { self.0 }
}

impl<V, T> Validate for With<V, T>
where
    V: Validator<T> + Default,
{
    fn validate(&self) -> Result<(), ValidationError> {
        // Delegate to the provided validator type `V`.
        let v = V::default();
        v.validate(&self.0)
    }
}

#[cfg(feature = "async")]
impl<V, T> ValidateAsync for With<V, T>
where
    V: AsyncValidator<T> + Default,
{
    fn validate_async(&self) -> Pin<Box<dyn Future<Output = Result<(), ValidationError>> + '_>> {
        Box::pin(async move {
            let v = V::default();
            v.validate_async(&self.0).await
        })
    }
}