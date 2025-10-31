use validator::prelude::*;
use validator::validators::email_validator::Email;
use validator::validators::positive_number_validator::Positive;

#[derive(validator::Validate)]
struct User {
    #[validate(Email)]
    email: String,
    #[validate(Positive)]
    age: i32,
    // ignored (no attribute)
    note: String,
}

#[test]
fn user_valid_ok() {
    let u = User {
        email: "john@example.com".to_string(),
        age: 30,
        note: "n/a".to_string(),
    };
    assert!(validator::validate::Validate::validate(&u).is_ok());
}

#[test]
fn user_invalid_email_first() {
    let u = User {
        email: "invalid@".to_string(),
        age: 30,
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::InvalidEmail)
    );
}

#[test]
fn user_invalid_age() {
    let u = User {
        email: "john@example.com".to_string(),
        age: 0,
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::MustBePositive)
    );
}