#[cfg(test)]
use crate::prelude::*;
use crate::validators::MinLength;

#[test]
fn validate_max_length() {
    let v = MinLength::new(10);
    assert!(
        v.validate(&"0123456789".to_string()).is_ok(),
        "Length of 10 is ok"
    );
    assert_eq!(
        v.validate(&"123456789".to_string()),
        Err(ValidationError::MinLength(10)),
        "Length of 9 is not ok"
    );
}

#[test]
fn validate_variable_max_length() {
    let v = MinLength::new(5);
    assert!(v.validate(&"12345".to_string()).is_ok());
    assert_eq!(v.validate(&"1234".to_string()), Err(ValidationError::MinLength(5)));
}