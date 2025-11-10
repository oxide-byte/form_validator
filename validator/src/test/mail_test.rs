#[cfg(test)]
use crate::prelude::*;
use crate::validators::Email;

#[test]
fn email_validator_valid_and_invalid_samples() {
    let email = Email::default();

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
    let err = email.validate(&"invalid@".to_string()).unwrap_err();
    assert_eq!(err.code, "email");
    let err = email.validate(&"no-at-symbol".to_string()).unwrap_err();
    assert_eq!(err.code, "email");
    let e = email.validate(&"spaces not allowed@example.com".to_string()).unwrap_err();
    assert_eq!(e.code, "email");
}

#[test]
fn email_validator_more_samples() {
    let email = Email::default();

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
    let e = email.validate(&"Abc.example.com".to_string()).unwrap_err();
    assert_eq!(e.code, "email");
    let e = email.validate(&"A@b@c@example.com".to_string()).unwrap_err();
    assert_eq!(e.code, "email");
    let e = email.validate(&" leadingwhitespace@example.com".to_string()).unwrap_err();
    assert_eq!(e.code, "email");
    let e = email.validate(&"trailingwhitespace@example.com ".to_string()).unwrap_err();
    assert_eq!(e.code, "email");
}