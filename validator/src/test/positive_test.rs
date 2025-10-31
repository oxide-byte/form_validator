#[cfg(test)]
use crate::prelude::*;
use crate::validators::positive_number_validator::Positive;

#[test]
fn positive_validator_checks_numbers() {
    let v = Positive;

    assert!(v.validate(&5).is_ok(), "5 should be valid (> 0)");
    assert_eq!(
        v.validate(&0),
        Err(ValidationError::MustBePositive),
        "0 should be invalid"
    );
    assert_eq!(
        v.validate(&(-3)),
        Err(ValidationError::MustBePositive),
        "-3 should be invalid"
    );
    assert!(v.validate(&42).is_ok(), "42 should be valid");
    assert!(v.validate(&i32::MAX).is_ok(), "i32::MAX should be valid");
    assert_eq!(
        v.validate(&i32::MIN),
        Err(ValidationError::MustBePositive),
        "i32::MIN should be invalid"
    );
}