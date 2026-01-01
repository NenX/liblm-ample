mod argment;
use std::ffi::OsString;

use clap::{CommandFactory, Parser};
use inquire::{Confirm, MultiSelect, Select, Text};

use crate::command::{
  self,
  argment::{Cli, Commands},
};

pub fn tt<I, T>(itr: I)
where
  I: IntoIterator<Item = T>,
  T: Into<OsString> + Clone,
{
  // let a:Vec<OsString> = itr.into_iter().map(|x|x.into()).collect();
  let args: Vec<OsString> = itr.into_iter().map(|x| x.into()).collect();
  println!("args: {:?}", args);
  let x = Cli::try_parse_from(args);

  match x {
    Ok(arg) => match arg.command {
      Commands::Install(arg) => println!("arg: Install port {}", arg.port),
      Commands::Build => println!("arg: Build"),
      Commands::Start => println!("arg: Start"),
      Commands::PickColor => println!("arg: PickColor"),
      Commands::FastGithub => println!("arg: FastGithub"),
    },
    Err(_) => {
      println!("bad args");
      let mut prompter = Select::new("请你选择", Commands::to_vec());
      prompter.help_message = Some("提示：使用👆/👇箭头导航，输入文字过滤，按回车键选择。");
      let project = prompter.prompt().unwrap();
      match project {
        Commands::Install(_) => {
          let a = MultiSelect::new("请输入 port", Vec::from(["aa", "bb"]))
            .prompt()
            .unwrap();
          println!("你选择了：{}, port {:?}", project, a);
        }
        Commands::Build => println!("你选择了：{}", project),
        Commands::Start => println!("你选择了：{}", project),
        Commands::PickColor => println!("你选择了：{}", project),
        Commands::FastGithub => println!("你选择了：{}", project),
      }
    }
  }
}
