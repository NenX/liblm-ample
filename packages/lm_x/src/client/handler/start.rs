use std::path::Path;

use tar::Entry;
use tokio::fs::{self, ReadDir};

use crate::util::{
  CONFIG_FILE, MyResult, copy_static, dot_env_to_map_new, format_date_time_underscore, pre_work, run_command_spawn, run_command_spawn_envs
};

pub async fn do_start() -> MyResult<()> {


  let (env_m, _) = pre_work(false).await?;


  let mut start_task = run_command_spawn_envs(
    &format!("rspack serve -c {}", CONFIG_FILE),
    // "dir",
    env_m,
  )
  .await?;

  if !start_task.wait().await?.success() {
    return Err("启动失败！".into());
  }

  Ok(())
}

