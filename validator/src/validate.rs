use crate::engine::error::ValidationError;
use crate::engine::validator::Validator;
use core::marker::PhantomData;
use std::collections::HashMap;

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