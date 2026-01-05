use clap::Parser;
use napi_derive::napi;

use crate::{
  client::{
    argment::{Cli, SubCmd},
    handler::{do_build, do_start, doctor_check, doctor_rm_deps},
  },
  util::MyResult,
};

#[napi(object)]
pub struct A {
  pub name: String,
}

pub async fn handle(args: Vec<String>) -> MyResult<A> {
  let cli = Cli::parse_from(args);
  match cli.command {
    SubCmd::Install(arg) => {
      println!("arg: Install port {}", arg.port);
    }
    SubCmd::Build =>  do_build(false).await?,
    SubCmd::Build2 =>  do_build(true).await?,
    SubCmd::Start => do_start().await?,
    SubCmd::DoctorRm => doctor_rm_deps().await?,
    SubCmd::Doctor => doctor_check().await?,
  };
  Ok(A {
    name: "haha".to_string(),
  })
}
