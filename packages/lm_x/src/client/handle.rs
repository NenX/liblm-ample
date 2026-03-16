use crate::{
  client::{
    argment::SubCmd,
    handler::{do_build, do_install, do_libdev, do_start, doctor_check, doctor_rm_deps},
  },
  util::MyResult,
};

pub async fn do_handle(cmd: &SubCmd) -> MyResult<()> {
  match cmd {
    SubCmd::Install => do_install().await?,
    SubCmd::Build => do_build().await?,
    SubCmd::Start => do_start().await?,
    SubCmd::DoctorRm => doctor_rm_deps().await?,
    SubCmd::Doctor => doctor_check().await?,
    SubCmd::LibDev => do_libdev().await?,
  };
  Ok(())
}
