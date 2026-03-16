#![deny(clippy::all)]

mod client;
mod util;

use napi_derive::napi;

#[napi]
pub fn plus_100(input: u32) -> u32 {
  input + 100
}

#[napi]
pub async fn cli(args: Vec<String>) -> client::A {
  client::cli_handler(args).await.unwrap()
}
#[napi]
pub async fn prompt() {
  client::prompt_handle().await.unwrap()
}
