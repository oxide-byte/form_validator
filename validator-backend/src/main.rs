use validator::prelude::Validator;
use validator::print_check;
use validator::validate::Validate as ValidateTrait;
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
    let not_allowed_chars = NotAllowedChars::new(vec!["Z".to_string()]);

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
    #[derive(validator::Validate)]
    struct User {
        #[validate(Email)]
        email: String,
        #[validate(Positive)]
        age: i32,
        display_name: String,
    }

    let good = User {
        email: "jane.doe@example.com".into(),
        age: 30,
        display_name: "Jane".into(),
    };
    match good.validate() {
        Ok(()) => println!("User(good) is valid"),
        Err(e) => println!("User(good) is invalid: {}", e),
    }

    let bad = User {
        email: "not-an-email".into(),
        age: 0,
        display_name: "JD".into(),
    };
    match bad.validate() {
        Ok(()) => println!("User(bad) is valid"),
        Err(e) => println!("User(bad) is invalid: {}", e),
    }
}