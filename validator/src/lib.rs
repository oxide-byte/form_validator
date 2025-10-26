use crate::validators::validator::Validator;

pub mod validators;

pub fn print_check<T, V, FOK, FERR>(v: &V, value: &T, on_ok: FOK, on_err: FERR)
where
    V: Validator<T>,
    FOK: Fn(&T),
    FERR: Fn(&T, &str),
{
    match v.validate(value) {
        Ok(()) => on_ok(value),
        Err(e) => on_err(value, &e),
    }
}