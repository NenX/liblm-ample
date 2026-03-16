use crate::util::{MyResult, copy_envjs, copy_static, run_command_spawn};

pub async fn do_libdev() -> MyResult<()> {
  // let mut start_task = run_command_spawn("cd .. && dir").await?;
  let is_existed = tokio::fs::try_exists("../liblm").await?;
  if !is_existed {
    let mut clone_task =
      run_command_spawn("cd .. && git clone https://github.com/NenX/liblm.git").await?;

    if !clone_task.wait().await?.success() {
      return Err("拉取 liblm 仓库失败".into());
    }
  }
  let is_existed = tokio::fs::try_exists("scripts/pnpm-workspace.yaml").await?;
  if !is_existed {
    return Err("pnpm-workspace.yaml 不存在".into());
  }

  tokio::fs::copy("scripts/pnpm-workspace.yaml", "pnpm-workspace.yaml").await?;

  let mut install_task = run_command_spawn("pnpm i").await?;

  if !install_task.wait().await?.success() {
    return Err("安装链接失败".into());
  }
  tokio::fs::remove_file("pnpm-workspace.yaml").await?;

  let is_existed = tokio::fs::try_exists("../liblm/node_modules").await?;
  if !is_existed {
    let mut install_task = run_command_spawn("cd ../liblm && pnpm i").await?;

    if !install_task.wait().await?.success() {
      return Err("安装 liblm 依赖失败".into());
    }
  }
  Ok(())
}
