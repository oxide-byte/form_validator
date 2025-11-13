use validator::prelude::*;
use validator::validators::*;

#[derive(validator::Validate)]
struct MultiError {
    #[validate(MinLength(3), NotAllowedChars(["C"]))]
    first_name: String,
    #[validate(MinLength(3), NotAllowedChars(["C"]))]
    last_name: String,
}

#[test]
fn range_max_error() {
    let u = MultiError {
        first_name: "AC".to_string(),
        last_name: "AB".to_string(),
    };
    let err = validator::validate::Validate::complete_validate(&u).unwrap_err();
    assert_eq!(err.len(), 2);
    assert_eq!(err.get("first_name").map(|v| v.len()), Some(2));
    assert_eq!(err.get("last_name").map(|v| v.len()), Some(1));
}