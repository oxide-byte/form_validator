use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ValidationError {
    #[error("Invalid email format")]
    InvalidEmail,
    #[error("Value must be a positive number")]
    MustBePositive,
    #[error("Value contains not allowed characters")]
    NotAllowedChars(String),
    #[error("Value is not valid")]
    NotValid,
}
