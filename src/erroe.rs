use thiserror::Error;

#[derive(Debug, Error, PartialEq)]
pub enum AppError {
  #[error("ParseError: {0}")] ParseError(String),
  #[error("SyntaxError: {0}")] SyntaxError(String),
}

pub type AppResult<T> = Result<T, AppError>;
