use validator::prelude::*;
use validator::validators::MinLength;

#[test]
fn validate_max_length() {
    let v = MinLength::new(10);
    assert!(
        v.validate(&"0123456789".to_string()).is_ok(),
        "Length of 10 is ok"
    );
    let err = v.validate(&"123456789".to_string()).unwrap_err();
    assert_eq!(err.code, "min_length");
    assert_eq!(err.params.get("limit").map(std::borrow::Cow::as_ref), Some("10"));
    assert_eq!(err.params.get("len").map(std::borrow::Cow::as_ref), Some("9"));
}

#[test]
fn validate_variable_max_length() {
    let v = MinLength::new(5);
    assert!(v.validate(&"12345".to_string()).is_ok());
    let err = v.validate(&"1234".to_string()).unwrap_err();
    assert_eq!(err.code, "min_length");
    assert_eq!(err.params.get("limit").map(std::borrow::Cow::as_ref), Some("5"));
    assert_eq!(err.params.get("len").map(std::borrow::Cow::as_ref), Some("4"));
}