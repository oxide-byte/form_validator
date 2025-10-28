use validator::print_check;
use validator::validators::email_validator::Email;
use validator::validators::not_allowed_chars::NotAllowedChars;
use validator::validators::positive_number_validator::Positive;

fn main() {
    let positive = Positive;
    let number_samples = [5, 0, -3, 42];
    for n in number_samples {
        print_check(
            &positive,
            &n,
            |v| println!("{} is valid (greater than 0)", v),
            |v, e| println!("{} is invalid: {}", v, e),
        );
    }

    let email = Email;
    let not_allowed_chars = NotAllowedChars;

    let email_samples: Vec<String> = vec![
        "john@example.com".to_string(),
        "alice.smith+tag@sub.example.co.uk".to_string(),
        "invalid@".to_string(),
        "no-at-symbol".to_string(),
        "spaces not allowed@example.com".to_string(),
        "ZTeam@example.com".to_string(),
    ];

    for s in &email_samples {
        print_check(
            &email,
            s,
            |v| println!("'{}' is a valid email", v),
            |v, e| println!("'{}' is an invalid email: {}", v, e),
        );

        print_check(
            &not_allowed_chars,
            s,
            |v| println!("'{}' passed not-allowed-chars check", v),
            |v, e| println!("'{}' failed not-allowed-chars check: {}", v, e),
        );
    }
}
