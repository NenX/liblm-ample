use std::ffi::OsStr;

use tokio::process::{Child, Command};

use crate::util::MyResult;

pub fn platform_cmd<S: AsRef<OsStr>>(cmd: S) -> Command {
  if cfg!(target_os = "windows") {
    let mut c = Command::new("cmd");
    c.arg("/C").arg(cmd);
    c
  } else {
    let mut c = Command::new("sh");
    c.arg("-c").arg(cmd);
    c
  }
}

pub async fn run_command(cmd_str: &str) -> MyResult<String> {
  let output = platform_cmd(cmd_str).output().await?;

  if output.status.success() {
    let out = output.stdout;
    let output_str = String::from_utf8_lossy(&out);
    Ok(output_str.to_string())
  } else {
    let out = output.stderr;
    let output_str = String::from_utf8_lossy(&out);
    Err(output_str.to_string().into())
  }
}
pub async fn run_command_spawn(cmd_str: &str) -> std::io::Result<Child> {
  let c = platform_cmd(cmd_str);
  let c = config_cmd_io(c).spawn()?;

  Ok(c)
}

pub async fn run_command_spawn_envs(
  cmd_str: &str,
  vars: impl IntoIterator<Item = (&str, &str)>,
) -> std::io::Result<Child> {
  let c = platform_cmd(cmd_str);
  let c = config_cmd_io(c).envs(vars).spawn()?;

  Ok(c)
}
fn config_cmd_io(mut cmd: Command) -> Command {
  cmd
    .stdin(std::process::Stdio::piped())
    // napi 在 linux 下 child process stdout inherit 有问题
    // .stdout(std::process::Stdio::inherit())
    // .stderr(std::process::Stdio::inherit());
    .stdout(std::io::stdout())
    .stderr(std::io::stderr());

  cmd
}

#[tokio::test]
async fn aa() {
  let a = run_command("pnpm --version").await;
  println!("aa{:?}", a)
}
