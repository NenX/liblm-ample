use std::collections::HashMap;

use chrono::Local;

use crate::util::{CheckVersion, error::MyResult, run_command, run_command_spawn};

pub const CONFIG_FILE: &str = "node_modules/@lm_fe/scripts/assets/config.js";

pub fn format_date_time() -> String {
  let now = Local::now();

  now.format("%Y-%m-%d %H:%M:%S").to_string()
}
pub fn format_date_time_underscore() -> String {
  let now = Local::now();

  now.format("%y_%m%d_%H%M").to_string()
}

pub async fn copy_envjs() -> MyResult<()> {
  if !tokio::fs::try_exists("env.js").await? {
    tokio::fs::copy("node_modules/@lm_fe/scripts/assets/env.js", "env.js").await?;
  }
  Ok(())
}
pub async fn dot_env_to_map_new() -> MyResult<HashMap<String, String>> {
  let code = if cfg!(windows) {
    r#"node -e console.log(JSON.stringify(require('./env.js')))"#.to_string()
  } else {
    r#"node -e "console.log(JSON.stringify(require('./env.js')))""#.to_string()
  };

  let json_str = run_command(&code).await?;
  let map: HashMap<String, String> = serde_json::from_str(&json_str)?;

  Ok(map)
}

pub fn simple_encrypt_str(data: &str) -> String {
  let aa = data
    .split("")
    .filter(|x| !x.is_empty())
    .enumerate()
    .filter_map(|(idx, x)| {
      let b = x.bytes().next().unwrap();
      let res = !(b as isize) + (idx as isize) * 119;
      Some(res.to_string())
    });
  let aa: Vec<_> = aa.collect();

  return aa.join("@@");
}

pub async fn pre_work(dev_mod: bool) -> MyResult<(HashMap<String, String>, CheckVersion)> {
  let mut env_m = dot_env_to_map_new().await?;
  let mut check_v = CheckVersion::new("public", "dist").await;
  if env_m.contains_key("HOST_URL") {
    let host_url = env_m.get("HOST_URL").unwrap();

    env_m.insert("LM_HOST_URL".into(), simple_encrypt_str(&host_url));
  }

  env_m.insert(
    "check_version".into(),
    check_v.write_next().await?.n.to_string(),
  );
  env_m.insert("LM_BUILD_AT".into(), format_date_time_underscore());

  let mode = if dev_mod { "development" } else { "production" };
  env_m.insert("ENVIRONMENT_MODE".into(), mode.into());

  Ok((env_m, check_v))
}
pub async fn mov_public_items() -> MyResult<()> {
  let public_tar_name = "public.tar.gz";
  let public_tar_path = std::path::Path::new(public_tar_name);
  if !public_tar_path.exists() {
    let cmd = &format!("cd public && tar -czf ../{} ./*", public_tar_name);

    let mut c = run_command_spawn(cmd).await?;
    c.wait().await?;
  }
  let cmd = &format!("tar -xzf {} -C dist", public_tar_name);

  let mut c = run_command_spawn(cmd).await?;
  c.wait().await?;

  Ok(())
}

pub async fn copy_static() -> MyResult<()> {
  let target_gz = "node_modules/@lm_fe/static/all.tar.gz";
  let a = public_contains_lm_static().await?;
  let b = !std::path::Path::new(target_gz).exists();
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
