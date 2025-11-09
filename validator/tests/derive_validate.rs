use validator::prelude::*;
use validator::validators::*;

#[derive(validator::Validate)]
struct User {
    #[validate(MaxLength(10))]
    name: String,
    #[validate(MinLength(10))]
    address: String,
    #[validate(NotAllowedChars(["CHEF"]))]
    role: String,
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
        name: "Test".to_string(),
        email: "john@example.com".to_string(),
        address: "Boulevard Null Pointer".to_string(),
        age: 30,
        role: "Admin".to_string(),
        note: "n/a".to_string(),
    };
    assert!(validator::validate::Validate::validate(&u).is_ok());
}

#[test]
fn user_invalid_email_first() {
    let u = User {
        name: "Test".to_string(),
        email: "invalid@".to_string(),
        address: "Boulevard Null Pointer".to_string(),
        age: 30,
        role: "Admin".to_string(),
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
        name: "Test".to_string(),
        email: "john@example.com".to_string(),
        address: "Boulevard Null Pointer".to_string(),
        age: 0,
        role: "Admin".to_string(),
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::MustBePositive)
    );
}

#[test]
fn user_invalid_length() {
    let u = User {
        name: "Test1234567890".to_string(),
        email: "john@example.com".to_string(),
        address: "Boulevard Null Pointer".to_string(),
        age: 30,
        role: "Admin".to_string(),
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::MaxLength(10))
    );
}

#[test]
fn address_invalid_length() {
    let u = User {
        name: "Test".to_string(),
        email: "john@example.com".to_string(),
        address: "NULL".to_string(),
        age: 30,
        role: "Admin".to_string(),
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::MinLength(10))
    );
}

#[test]
fn user_not_allowed_chars() {
    let u = User {
        name: "Test".to_string(),
        email: "john@example.com".to_string(),
        address: "Boulevard Null Pointer".to_string(),
        age: 30,
        role: "CHEF".to_string(),
        note: "n/a".to_string(),
    };
    assert_eq!(
        validator::validate::Validate::validate(&u),
        Err(ValidationError::NotAllowedChars("CHEF".to_string()))
    );
}
