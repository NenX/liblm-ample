use std::path::Path;

use tar::Entry;
use tokio::fs::{self, ReadDir};

use crate::util::{
  CONFIG_FILE, MyResult, copy_envjs, copy_static, dot_env_to_map_new, format_date_time_underscore,
  pre_work, run_command_spawn, run_command_spawn_envs,
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
