use std::process;

use inquire::Select;

use crate::{
  client::{
    argment::SubCmd,
    handle::{self, do_handle},
    handler::{do_build, do_install, do_libdev, do_start, doctor_check, doctor_rm_deps},
  },
  util::MyResult,
};

pub async fn prompt_handle() -> MyResult<()> {
  let mut prompter = Select::new("请你选择", SubCmd::to_vec());
  prompter.help_message = Some("提示：使用👆/👇箭头导航，输入文字过滤，按回车键选择。");
  let Ok(project) = prompter.prompt() else {
    println!();
    process::exit(0)
  };
  do_handle(&project).await?;
  Ok(())
}
