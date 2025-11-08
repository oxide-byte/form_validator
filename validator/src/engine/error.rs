use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Value must be a positive number")]
    MustBePositive,
    #[error("Value contains not allowed string")]
    NotAllowedChars(String),
    #[error("String to long")]
    MaxLength(u32),
    #[error("String to short")]
    MinLength(u32),
    #[error("Value is not valid")]
    NotValid,
}