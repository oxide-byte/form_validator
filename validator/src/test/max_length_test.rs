#[cfg(test)]
use crate::prelude::*;
use crate::validators::MaxLength;

#[test]
fn validate_max_length() {
    let v = MaxLength::new(10);
    assert!(
        v.validate(&"0123456789".to_string()).is_ok(),
        "Length of 10 is ok"
    );
    assert_eq!(
        v.validate(&"0123456789123".to_string()),
        Err(ValidationError::MaxLength(10)),
        "Length of 13 is not ok"
    );
}

#[test]
fn validate_variable_max_length() {
    let v = MaxLength::new(5);
    assert!(v.validate(&"12345".to_string()).is_ok());
    assert_eq!(v.validate(&"123456".to_string()), Err(ValidationError::MaxLength(5)));
}