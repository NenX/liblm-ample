use crate::util::{MyResult, run_command, run_command_spawn};

pub async fn doctor_check() -> MyResult<()> {
  let pnpm_status = run_command("pnpm -v").await;
  let pm2_status = run_command("pm2 -v").await;
  let rspack_status = run_command("rspack -v").await;
  if rspack_status.is_ok() && pnpm_status.is_ok() && pm2_status.is_ok() {
    println!("检测成功！");
    return Ok(());
  }

  let ans = inquire::Confirm::new("缺少部分关键依赖，是否现在安装？")
    .with_default(true)
    .prompt();

  if let Ok(yes) = ans
    && yes
  {
    let mut deps = String::from("pnpm -g install");
    if pnpm_status.is_err() {
      let mut child = run_command_spawn("npm install -g pnpm").await?;
      child.wait().await?;
    }
    if pm2_status.is_err() {
      deps.push_str(" pm2");
    }
    if rspack_status.is_err() {
      deps.push_str(" @rspack/cli@1.6.8 @rspack/core@1.6.8 @rspack/plugin-react-refresh@1.5.1");
    }

    let mut child = run_command_spawn(&deps).await?;
    child.wait().await?;
  }

  Ok(())
}
pub async fn doctor_rm_deps() -> MyResult<()> {
  let mut child = run_command_spawn("pnpm rm -g pm2 @rspack/cli @rspack/core @rspack/plugin-react-refresh").await?;
  child.wait().await?;

  Ok(())
}
