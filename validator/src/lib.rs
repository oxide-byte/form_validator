pub mod validators;
pub mod validate;
pub mod prelude;
mod engine;
mod test;

use crate::engine::error::ValidationError;
use crate::engine::validator::Validator;
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