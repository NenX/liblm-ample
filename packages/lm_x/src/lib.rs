#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub fn hello(name: Option<String>, input: Option<i32>) -> String {
  format!(
    "hello {}, input is {}",
    name.unwrap_or("default".into()),
    input.unwrap_or(999)
  )
}
