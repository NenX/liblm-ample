use std::num::ParseIntError;

use thiserror::Error;
use tokio::io;

#[derive(Error, Debug)]
pub enum MyError {
  #[error("data store disconnected")]
  Io(#[from] io::Error),
  #[error("data store disconnected")]
  NumParse(#[from] ParseIntError),
  #[error("the data for key `{0}` is not available")]
  Msg(String),
  #[error("invalid header (expected {expected:?}, found {found:?})")]
  InvalidHeader { expected: String, found: String },
  #[error("unknown data store error")]
  Unknown,
}
pub type MyResult<T> = Result<T, MyError>;

impl From<&str> for MyError {
  fn from(value: &str) -> Self {
    MyError::Msg(value.to_owned())
  }
}

impl From<String> for MyError {
  fn from(value: String) -> Self {
    MyError::Msg(value)
  }
}


