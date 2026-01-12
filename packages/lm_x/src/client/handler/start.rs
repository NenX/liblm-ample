use std::path::Path;

use tar::Entry;
use tokio::fs::{self, ReadDir};

use crate::util::{
  MyResult, dot_env_to_map_new, format_date_time_underscore, pre_work, run_command_spawn, run_command_spawn_envs
};

pub async fn do_start() -> MyResult<()> {
  copy_static().await?;

  let (env_m, _) = pre_work(false).await?;
  let name = env_m.get("APP_KEY").cloned().unwrap();

  println!("开始运行: {:?}", env_m);

  let mut start_task = run_command_spawn_envs(
    "rspack serve",
    // "dir",
    env_m,
  )
  .await?;

  if !start_task.wait().await?.success() {
    return Err("启动失败！".into());
  }

  Ok(())
}

async fn copy_static() -> MyResult<()> {
  let target_gz = "node_modules/@lm_fe/static/all.tar.gz";
  let a = public_contains_lm_static().await?;
  let b = !Path::new(target_gz).exists();
  println!("copy static {a} {b}");
  if a || b {
    return Ok(());
  }
  let cmd = &format!("tar -xzf {} -C public", target_gz);
  println!("cmd => {cmd}");
  let mut c = run_command_spawn(cmd).await?;
  c.wait().await?;

  Ok(())
}
async fn public_contains_lm_static() -> MyResult<bool> {
  use tokio_stream::StreamExt;
  use tokio_stream::wrappers::ReadDirStream;
  let dir = tokio::fs::read_dir("public").await?;
  let result = ReadDirStream::new(dir)
    .any(|res| {
      res
        .unwrap()
        .file_name()
        .to_str()
        .map_or(false, |x| x.starts_with("lm_"))
    })
    .await;
  Ok(result)
}
