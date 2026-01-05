use flate2::{Compression, write::GzEncoder};
use fs_extra::dir::CopyOptions;

use std::path::Path;
use tokio::{fs, time::Instant};

use crate::util::{
  CheckVersion, MyResult, dot_env_to_map_new, format_date_time_underscore, run_command,
  run_command_spawn, run_command_spawn_envs,
};

const PACK_DIR: &str = "lm_packet";
const LATEST_PACK: &str = "latest";

pub async fn do_build() -> MyResult<()> {
  let mut env_m = dot_env_to_map_new().await?;
  let mut check_v = CheckVersion::new("public", "dist").await;

  env_m.insert(
    "check_version".to_string(),
    check_v.write_next().await?.n.to_string(),
  );
  env_m.insert("LM_BUILD_AT".to_string(), format_date_time_underscore());
  println!("开始构建: {:?}", env_m);

  let mut build_task = run_command_spawn_envs(
    "rspack build",
    env_m.iter().map(|(a, b)| (a.as_str(), b.as_str())),
  )
  .await?;
  let start = Instant::now();

  let name = env_m.get("APP_KEY").cloned().unwrap_or("".to_string());

  let gz_path = format!(r"{}_{}.tar.gz", name, format_date_time_underscore());

  if !build_task.wait().await?.success() {
    return Err("haha".into());
  }
  mov_the_fucking_things().expect("Failed to move files");
  check_v.write_to().await?;

  compress_dist(&Path::new(PACK_DIR).join(&gz_path)).await?;
  fs::write(Path::new(PACK_DIR).join(LATEST_PACK), gz_path).await?;
  println!("操作成功！耗时 {:?}", start.elapsed());

  Ok(())
}

pub async fn compress_dist(name: &Path) -> MyResult<()> {
  if cfg!(windows) {
    let dir_path = Path::new(PACK_DIR);
    if !dir_path.is_dir() {
      fs::create_dir_all(PACK_DIR).await?;
    }

    let tar_gz = std::fs::File::create(name)?;
    let enc = GzEncoder::new(tar_gz, Compression::default());
    let mut tar = tar::Builder::new(enc);
    tar.append_dir_all(".", "dist")?;
    tar.finish()?;
    let _ = run_command(&format!("explorer {}", PACK_DIR)).await;
  } else {
    let cmd = &format!("tar -czvf -C dist {}", name.to_str().unwrap());
    println!("compress cmd {}", cmd);
    let mut c = run_command_spawn(&format!("tar -czvf -C dist {}", cmd)).await?;
    c.wait().await?;
  }
  Ok(())
}
fn mov_the_fucking_things() -> fs_extra::error::Result<()> {
  let mut options = CopyOptions::new();
  options.overwrite = true;
  options.content_only = true;
  options.skip_exist = true;
  // let from_paths = vec![
  //     "public/img"
  // ];
  // fs_extra::copy_items(&from_paths, target, &options)?;
  fs_extra::dir::copy("public", "dist", &options)?;

  Ok(())
}
