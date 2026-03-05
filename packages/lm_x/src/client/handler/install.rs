

use crate::util::{
  MyResult, copy_envjs, copy_static, run_command_spawn,
};

pub async fn do_install() -> MyResult<()> {
  let mut start_task = run_command_spawn("pnpm i").await?;

  if start_task.wait().await?.success() {
    copy_static().await?;
    copy_envjs().await?;
  } else {
    return Err("启动失败！".into());
  }

  Ok(())
}
