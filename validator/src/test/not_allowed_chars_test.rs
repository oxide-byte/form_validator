#[cfg(test)]
use crate::prelude::*;
use crate::validators::NotAllowedChars;

#[test]
fn not_allowed_chars_validator_empty_string() {
    let v = NotAllowedChars::new(vec!["Z".to_string()]);
    assert!(v.validate(&"".to_string()).is_ok(), "Empty string should be valid");
}

#[test]
fn not_allowed_chars_validator_checks() {
    let v = NotAllowedChars::new(vec!["Z".to_string()]);

    // Should be ok (no 'Z')
    assert!(
        v.validate(&"john@example.com".to_string()).is_ok(),
        "'john@example.com' should pass not-allowed-chars check"
    );
    assert!(
        v.validate(&"alice.smith+tag@sub.example.co.uk".to_string()).is_ok(),
        "'alice.smith+tag@sub.example.co.uk' should pass not-allowed-chars check"
    );
    assert!(
        v.validate(&"invalid@".to_string()).is_ok(),
        "'invalid@' should pass not-allowed-chars check"
    );
    assert!(
        v.validate(&"no-at-symbol".to_string()).is_ok(),
        "'no-at-symbol' should pass not-allowed-chars check"
    );
    assert!(
        v.validate(&"spaces not allowed@example.com".to_string()).is_ok(),
        "'spaces not allowed@example.com' should pass not-allowed-chars check"
    );

    // Should fail (contains 'Z')
    assert_eq!(
        v.validate(&"ZTeam@example.com".to_string()),
        Err(ValidationError::NotAllowedChars("Z".to_string())),
        "'ZTeam@example.com' should fail not-allowed-chars check"
    );

    let v = NotAllowedChars::new(vec!["P".to_string()]);

    // Should fail (contains 'P')
    assert_eq!(
        v.validate(&"Player1".to_string()),
        Err(ValidationError::NotAllowedChars("P".to_string())),
        "'Player1' should fail not-allowed-chars check"
    );
}