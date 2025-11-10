use validator::prelude::*;
use validator::validators::*;

#[derive(validator::Validate)]
struct StringRange {
    #[validate(MinLength(3), MaxLength(10))]
    name: String,
}

#[test]
fn range_ok() {
    let u = StringRange {
        name: "Test".to_string(),
    };
    assert!(validator::validate::Validate::validate(&u).is_ok());
}

#[test]
fn range_min_error() {
    let u = StringRange {
        name: "T".to_string(),
    };
    let err = validator::validate::Validate::validate(&u).unwrap_err();
    assert_eq!(err.code, "min_length");
}

#[test]
fn range_max_error() {
    let u = StringRange {
        name: "Test1234567890".to_string(),
    };
    let err = validator::validate::Validate::validate(&u).unwrap_err();
    assert_eq!(err.code, "max_length");
}