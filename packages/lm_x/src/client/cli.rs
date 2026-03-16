use clap::Parser;
use napi_derive::napi;

use crate::{
  client::{
    argment::{Cli, SubCmd},
    handle::do_handle,
    handler::{do_build, do_install, do_libdev, do_start, doctor_check, doctor_rm_deps},
  },
  util::MyResult,
};

#[napi(object)]
pub struct A {
  pub name: String,
}

pub async fn cli_handler(args: Vec<String>) -> MyResult<A> {
  let cli = Cli::parse_from(args);
  do_handle(&cli.command).await?;
  Ok(A {
    name: "haha".to_string(),
  })
}
