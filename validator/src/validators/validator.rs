use regex::Regex;

pub trait Validator<T> {
    fn validate(&self, value: &T) -> Result<(), String>;
}

pub trait PositiveNumberValidator: Validator<i32> {}
pub struct Positive;

impl Validator<i32> for Positive {
    fn validate(&self, value: &i32) -> Result<(), String> {
        if *value > 0 {
            Ok(())
        } else {
            Err("Value must be greater than 0".to_string())
        }
    }
}

impl PositiveNumberValidator for Positive {}

pub trait EmailValidator: Validator<String> {}

pub struct Email;

impl Validator<String> for Email {
    fn validate(&self, value: &String) -> Result<(), String> {
        let re = Regex::new(r"^[^@\s]+@[^@\s]+\.[^@\s]+$").unwrap();
        if re.is_match(value) {
            Ok(())
        } else {
            Err("Invalid email format".to_string())
        }
    }
}

impl EmailValidator for Email {}

pub struct NotAllowedChars;

impl Validator<String> for NotAllowedChars {
    fn validate(&self, value: &String) -> Result<(), String> {
        if !value.contains("Z") {
            Ok(())
        } else {
            Err("Z not allowed".to_string())
        }
    }
}