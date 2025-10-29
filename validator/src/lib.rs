use crate::validators::error::ValidationError;
use crate::validators::validator::Validator;

pub mod validators;
pub mod validate;

// Re-export the derive macro so consumers can `use validator::Validate;`
pub use validator_derive::Validate;

pub fn print_check<T, V, FOK, FERR>(v: &V, value: &T, on_ok: FOK, on_err: FERR)
where
    V: Validator<T>,
    FOK: Fn(&T),
    FERR: Fn(&T, &ValidationError),
{
    match v.validate(value) {
        Ok(()) => on_ok(value),
        Err(e) => on_err(value, &e),
    }
}