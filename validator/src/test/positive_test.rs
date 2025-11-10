#[cfg(test)]
use crate::prelude::*;
use crate::validators::Positive;

#[test]
fn positive_validator_checks_numbers() {
    let v = Positive::default();

    assert!(v.validate(&5).is_ok(), "5 should be valid (> 0)");
    let err = v.validate(&0).unwrap_err();
    assert_eq!(err.code, "positive");
    let err = v.validate(&(-3)).unwrap_err();
    assert_eq!(err.code, "positive");
    assert!(v.validate(&42).is_ok(), "42 should be valid");
    assert!(v.validate(&i32::MAX).is_ok(), "i32::MAX should be valid");
    let err = v.validate(&i32::MIN).unwrap_err();
    assert_eq!(err.code, "positive");
}