use std::fmt::Display;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
  #[command(subcommand)]
  pub command: SubCmd,
}
#[derive(Args, Clone, Copy, Default, Debug)]
pub struct InstallArgs {
  /// 代理的本地端口
  #[arg(short, long)]
  pub port: u16,
  /// 暴露端口号
  #[arg(short, long)]
  pub export: Option<u16>,
}
#[derive(Subcommand, Clone, Debug)]
pub enum SubCmd {
  /// 安装依赖
  Install,
  /// 打包本地项目并压缩
  Build,
  Start,
  /// 取颜色
  DoctorRm,
  /// 更新本地 GitHub Host，加快访问速度
  Doctor,
  /// 库开发准备
  LibDev,
}

impl SubCmd {
  pub fn to_vec() -> Vec<Self> {
    vec![
      SubCmd::Start,
      SubCmd::Build,
      SubCmd::Install,
      SubCmd::Doctor,
      SubCmd::DoctorRm,
      SubCmd::LibDev,
    ]
  }
}
impl Display for SubCmd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SubCmd::Install => write!(f, "安装依赖"),
      SubCmd::Build => write!(f, "build"),
      SubCmd::Start => write!(f, "start"),
      SubCmd::DoctorRm => write!(f, "移除环境依赖"),
      SubCmd::Doctor => write!(f, "环境检测"),
      SubCmd::LibDev => write!(f, "库开发准备"),
    }
  }
}
