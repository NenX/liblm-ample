use std::process;

use inquire::Select;

use crate::{
  client::{
    argment::SubCmd,
    handler::{do_build, do_install, do_start, doctor_check, doctor_rm_deps},
  },
  util::MyResult,
};

pub async fn handle() -> MyResult<()> {
  let mut prompter = Select::new("请你选择", SubCmd::to_vec());
  prompter.help_message = Some("提示：使用👆/👇箭头导航，输入文字过滤，按回车键选择。");
  let Ok(project) = prompter.prompt() else {
    println!();
    process::exit(0)
  };

  match project {
    SubCmd::Install => {
      // let a = MultiSelect::new("请输入 port", Vec::from(["aa", "bb"]))
      //   .prompt()
      //   .expect("不会了");
      // println!("你选择了：{}, port {:?}", project, a);
      do_install().await?;
    }
    SubCmd::Build => do_build().await?,
    SubCmd::Start => do_start().await?,
    SubCmd::DoctorRm => doctor_rm_deps().await?,
    SubCmd::Doctor => doctor_check().await?,
  };
  Ok(())
}
