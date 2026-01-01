use std::fmt::Display;

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
  #[command(subcommand)]
  pub command: Commands,
}
#[derive(Args, Clone, Copy, Default)]
pub struct InstallArgs {
  /// 代理的本地端口
  #[arg(short, long)]
  pub port: u16,
  /// 暴露端口号
  #[arg(short, long)]
  pub export: Option<u16>,
}
#[derive(Subcommand, Clone)]
pub enum Commands {
  // /// 构建远程单个 lm_fe 依赖并安装到本地，用法: fresh pages-mchc
  // Fresh,
  /// 安装 lm_fe 依赖安装到本地，可指定单一依赖
  Install(InstallArgs),
  /// 打包本地项目并压缩
  Build,
  Start,
  /// 取颜色
  PickColor,
  /// 更新本地 GitHub Host，加快访问速度
  FastGithub,
}

impl Commands {
  pub fn to_vec() -> Vec<Self> {
    vec![
      Commands::Start,
      Commands::Build,
      Commands::Install(InstallArgs::default()),
      Commands::FastGithub,
      Commands::PickColor,
    ]
  }
}
impl Display for Commands {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Commands::Install(_) => write!(f, "哈哈 Install"),
      Commands::Build => write!(f, "哈哈 Build"),
      Commands::Start => write!(f, "哈哈 Start"),
      Commands::PickColor => write!(f, "哈哈 PickColor"),
      Commands::FastGithub => write!(f, "哈哈 FastGithub"),
    }
  }
}
