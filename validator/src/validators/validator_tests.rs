#[cfg(test)]
use crate::validators::validator::{Email, NotAllowedChars, Positive, Validator};

#[test]
fn positive_validator_checks_numbers() {
    let v = Positive;
    let samples = [5, 0, -3, 42];

    assert!(v.validate(&samples[0]).is_ok(), "5 should be valid (> 0)");
    assert!(v.validate(&samples[1]).is_err(), "0 should be invalid");
    assert!(v.validate(&samples[2]).is_err(), "-3 should be invalid");
    assert!(v.validate(&samples[3]).is_ok(), "42 should be valid");
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
    assert!(email.validate(&samples[0]).is_ok(), "john@example.com should be valid");
    assert!(email.validate(&samples[1]).is_ok(), "alice.smith+tag@sub.example.co.uk should be valid");
    assert!(email.validate(&samples[5]).is_ok(), "ZTeam@example.com should be valid by format");

    // Invalid emails
    assert!(email.validate(&samples[2]).is_err(), "invalid@ should be invalid");
    assert!(email.validate(&samples[3]).is_err(), "no-at-symbol should be invalid");
    assert!(email.validate(&samples[4]).is_err(), "spaces not allowed@example.com should be invalid");
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
    for i in 0..samples.len() - 1 {
        assert!(v.validate(&samples[i]).is_ok(), "'{}' should pass not-allowed-chars check", samples[i]);
    }
    // Should fail (contains 'Z')
    assert!(v.validate(&samples[5]).is_err(), "'ZTeam@example.com' should fail not-allowed-chars check");
}