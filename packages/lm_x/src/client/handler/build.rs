use flate2::{Compression, write::GzEncoder};
use fs_extra::dir::CopyOptions;

use std::path::Path;
use tokio::{fs, time::Instant};

use crate::util::{
  CONFIG_FILE, CheckVersion, MyResult, dot_env_to_map_new, format_date_time_underscore, mov_public_items, pre_work, run_command, run_command_spawn, run_command_spawn_envs
};

const PACK_DIR: &str = "lm_packet";
const LATEST_PACK: &str = "latest";

pub async fn do_build() -> MyResult<()> {
  let (env_m, check_v) = pre_work(false).await?;
  let name = env_m.get("APP_KEY").cloned().unwrap();


  let mut build_task =
    run_command_spawn_envs(&format!("rspack build -c {}", CONFIG_FILE), env_m).await?;
  let start = Instant::now();

  let gz_path = format!(r"{}_{}.tar.gz", name, format_date_time_underscore());

  if !build_task.wait().await?.success() {
    return Err("haha".into());
  }
  mov_public_items().await?;

  check_v.write_to().await?;

  compress_dist(&Path::new(PACK_DIR).join(&gz_path)).await?;
  fs::write(Path::new(PACK_DIR).join(LATEST_PACK), gz_path).await?;
  println!("操作成功！耗时 {:?}", start.elapsed());

  Ok(())
}

pub async fn compress_dist(name: &Path) -> MyResult<()> {
  let dir_path = Path::new(PACK_DIR);
  if !dir_path.is_dir() {
    fs::create_dir_all(PACK_DIR).await?;
  }

  let cmd = &format!("cd dist && tar -czf ../{} ./*", name.to_str().unwrap());

  let mut c = run_command_spawn(cmd).await?;
  c.wait().await?;

  Ok(())
}
