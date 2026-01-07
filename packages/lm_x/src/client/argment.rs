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
  /// 安装 lm_fe 依赖安装到本地，可指定单一依赖
  Install(InstallArgs),
  /// 打包本地项目并压缩
  Build,
  Start,
  /// 取颜色
  DoctorRm,
  /// 更新本地 GitHub Host，加快访问速度
  Doctor,
}

impl SubCmd {
  pub fn to_vec() -> Vec<Self> {
    vec![
      SubCmd::Start,
      SubCmd::Build,
      SubCmd::Install(InstallArgs::default()),
      SubCmd::Doctor,
      SubCmd::DoctorRm,
    ]
  }
}
impl Display for SubCmd {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      SubCmd::Install(_) => write!(f, "install"),
      SubCmd::Build => write!(f, "build"),
      SubCmd::Start => write!(f, "start"),
      SubCmd::DoctorRm => write!(f, "移除环境依赖"),
      SubCmd::Doctor => write!(f, "环境检测"),
    }
  }
}
