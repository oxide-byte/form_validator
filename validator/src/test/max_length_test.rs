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
    let err = v.validate(&"0123456789123".to_string()).unwrap_err();
    assert_eq!(err.code, "max_length");
    assert_eq!(err.params.get("limit").map(|s| s.as_ref()), Some("10"));
    assert_eq!(err.params.get("len").map(|s| s.as_ref()), Some("13"));
}

#[test]
fn validate_variable_max_length() {
    let v = MaxLength::new(5);
    assert!(v.validate(&"12345".to_string()).is_ok());
    let err = v.validate(&"123456".to_string()).unwrap_err();
    assert_eq!(err.code, "max_length");
    assert_eq!(err.params.get("limit").map(|s| s.as_ref()), Some("5"));
    assert_eq!(err.params.get("len").map(|s| s.as_ref()), Some("6"));
}