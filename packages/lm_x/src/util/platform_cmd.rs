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
  let c = platform_cmd(cmd_str)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .spawn()?;

  Ok(c)
}

pub async fn run_command_spawn_envs(
  cmd_str: &str,
  vars: impl IntoIterator<Item = (&str, &str)>,
) -> std::io::Result<Child> {
  let c = platform_cmd(cmd_str)
    .envs(vars)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .spawn()?;

  Ok(c)
}

#[tokio::test]
async fn aa() {
  let a = run_command("pnpm --version").await;
  println!("aa{:?}", a)
}
