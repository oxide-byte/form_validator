use crate::engine::error::ValidationError;
use crate::engine::validator::Validator;
use core::marker::PhantomData;

/// Trait for types that can validate themselves.
///
/// Implementations should return `Ok(())` when valid or a `ValidationError`
/// describing the first encountered problem.
pub trait Validate {
    fn validate(&self) -> Result<(), ValidationError>;
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