#[cfg(test)]
use crate::prelude::*;
use crate::validators::Email;

#[test]
fn email_validator_valid_and_invalid_samples() {
    let email = Email;

    // Valid emails
    assert!(
        email
            .validate(&"john@example.com".to_string())
            .is_ok(),
        "john@example.com should be valid"
    );
    assert!(
        email
            .validate(&"alice.smith+tag@sub.example.co.uk".to_string())
            .is_ok(),
        "alice.smith+tag@sub.example.co.uk should be valid"
    );
    assert!(
        email
            .validate(&"ZTeam@example.com".to_string())
            .is_ok(),
        "ZTeam@example.com should be valid by format"
    );

    // Invalid emails
    assert_eq!(
        email.validate(&"invalid@".to_string()),
        Err(ValidationError::InvalidEmail),
        "invalid@ should be invalid"
    );
    assert_eq!(
        email.validate(&"no-at-symbol".to_string()),
        Err(ValidationError::InvalidEmail),
        "no-at-symbol should be invalid"
    );
    assert_eq!(
        email.validate(&"spaces not allowed@example.com".to_string()),
        Err(ValidationError::InvalidEmail),
        "spaces not allowed@example.com should be invalid"
    );
}

#[test]
fn email_validator_more_samples() {
    let email = Email;

    // Valid samples
    assert!(
        email
            .validate(&"example_indeed@strange_example.com".to_string())
            .is_ok(),
        "example_indeed@strange_example.com should be a valid email"
    );
    assert!(
        email
            .validate(&"example-indeed@strange-example.com".to_string())
            .is_ok(),
        "example-indeed@strange-example.com should be a valid email"
    );

    // Invalid samples
    assert_eq!(
        email.validate(&"Abc.example.com".to_string()),
        Err(ValidationError::InvalidEmail),
        "Abc.example.com should be an invalid email"
    );
    assert_eq!(
        email.validate(&"A@b@c@example.com".to_string()),
        Err(ValidationError::InvalidEmail),
        "A@b@c@example.com should be an invalid email"
    );
    assert_eq!(
        email.validate(&" leadingwhitespace@example.com".to_string()),
        Err(ValidationError::InvalidEmail),
        " leadingwhitespace@example.com should be an invalid email"
    );
    assert_eq!(
        email.validate(&"trailingwhitespace@example.com ".to_string()),
        Err(ValidationError::InvalidEmail),
        "trailingwhitespace@example.com  should be an invalid email"
    );
}