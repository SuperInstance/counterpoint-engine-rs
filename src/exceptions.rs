//! Error types.
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CounterpointError {
    #[error("constraint violation: {0}")]
    ConstraintViolation(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("generation failed: {0}")]
    GenerationFailed(String),
}
