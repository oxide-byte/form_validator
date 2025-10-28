#[cfg(test)]
use crate::validators::email_validator::Email;
use crate::validators::not_allowed_chars::NotAllowedChars;
use crate::validators::positive_number_validator::Positive;
use crate::validators::validator::Validator;

use crate::validators::error::ValidationError;

#[test]
fn positive_validator_checks_numbers() {
    let v = Positive;
    let samples = [5, 0, -3, 42, i32::MAX, i32::MIN];

    assert!(v.validate(&samples[0]).is_ok(), "5 should be valid (> 0)");
    assert_eq!(
        v.validate(&samples[1]),
        Err(ValidationError::MustBePositive),
        "0 should be invalid"
    );
    assert_eq!(
        v.validate(&samples[2]),
        Err(ValidationError::MustBePositive),
        "-3 should be invalid"
    );
    assert!(v.validate(&samples[3]).is_ok(), "42 should be valid");
    assert!(v.validate(&samples[4]).is_ok(), "i32::MAX should be valid");
    assert_eq!(
        v.validate(&samples[5]),
        Err(ValidationError::MustBePositive),
        "i32::MIN should be invalid"
    );
}

#[test]
fn email_validator_valid_and_invalid_samples() {
    let email = Email;
    let samples: Vec<String> = vec![
        "john@example.com".to_string(),
        "alice.smith+tag@sub.example.co.uk".to_string(),
        "invalid@".to_string(),
        "no-at-symbol".to_string(),
        "spaces not allowed@example.com".to_string(),
        "ZTeam@example.com".to_string(),
    ];

    // Valid emails
    assert!(
        email.validate(&samples[0]).is_ok(),
        "john@example.com should be valid"
    );
    assert!(
        email.validate(&samples[1]).is_ok(),
        "alice.smith+tag@sub.example.co.uk should be valid"
    );
    assert!(
        email.validate(&samples[5]).is_ok(),
        "ZTeam@example.com should be valid by format"
    );

    // Invalid emails
    assert_eq!(
        email.validate(&samples[2]),
        Err(ValidationError::InvalidEmail),
        "invalid@ should be invalid"
    );
    assert_eq!(
        email.validate(&samples[3]),
        Err(ValidationError::InvalidEmail),
        "no-at-symbol should be invalid"
    );
    assert_eq!(
        email.validate(&samples[4]),
        Err(ValidationError::InvalidEmail),
        "spaces not allowed@example.com should be invalid"
    );
}

#[test]
fn email_validator_more_samples() {
    let email = Email;
    let valid_samples: Vec<String> = vec![
        "\"very.(),:;<>[]\\\".VERY.\\\"very@\\\\ \\\"very\\\".unusual\"@strange.example.com"
            .to_string(),
        "example-indeed@strange-example.com".to_string(),
    ];

    let invalid_samples: Vec<String> = vec![
        "Abc.example.com".to_string(),                            // No @
        "A@b@c@example.com".to_string(),                          // Multiple @
        "a\"b(c)d,e:f;g<h>i[j\\k]l@example.com".to_string(), // Special characters outside quotes
        "just\"not\"right@example.com".to_string(),          // Quoted text not allowed
        "this is\"not\\allowed@example.com".to_string(),     // Spaces outside quotes
        "this\\ still\\\"not\\\\allowed@example.com".to_string(), // Escaped characters outside quotes
        " leadingwhitespace@example.com".to_string(),
        "trailingwhitespace@example.com ".to_string(),
    ];

    for sample in valid_samples {
        assert!(
            email.validate(&sample).is_ok(),
            "'{}' should be a valid email",
            sample
        );
    }

    for sample in invalid_samples {
        assert_eq!(
            email.validate(&sample),
            Err(ValidationError::InvalidEmail),
            "'{}' should be an invalid email",
            sample
        );
    }
}

#[test]
fn not_allowed_chars_validator_empty_string() {
    let v = NotAllowedChars;
    let sample = "".to_string();
    assert!(v.validate(&sample).is_ok(), "Empty string should be valid");
}

#[test]
fn not_allowed_chars_validator_checks() {
    let v = NotAllowedChars;
    let samples: Vec<String> = vec![
        "john@example.com".to_string(),
        "alice.smith+tag@sub.example.co.uk".to_string(),
        "invalid@".to_string(),
        "no-at-symbol".to_string(),
        "spaces not allowed@example.com".to_string(),
        "ZTeam@example.com".to_string(),
    ];

    // Should be ok (no 'Z')
    for sample in samples.iter().take(samples.len() - 1) {
        assert!(
            v.validate(sample).is_ok(),
            "'{}' should pass not-allowed-chars check",
            sample
        );
    }
    // Should fail (contains 'Z')
    assert_eq!(
        v.validate(&samples[5]),
        Err(ValidationError::NotAllowedChars("Z".to_string())),
        "'ZTeam@example.com' should fail not-allowed-chars check"
    );
}
