use std::collections::HashMap;

use chrono::Local;

use crate::util::{CheckVersion, error::MyResult, run_command};

pub const CONFIG_FILE: &str = "node_modules/@lm_fe/scripts/assets/config.js";

pub fn format_date_time() -> String {
  let now = Local::now();

  now.format("%Y-%m-%d %H:%M:%S").to_string()
}
pub fn format_date_time_underscore() -> String {
  let now = Local::now();

  now.format("%y_%m%d_%H%M").to_string()
}

pub async fn dot_env_to_map_new() -> MyResult<HashMap<String, String>> {
  if !tokio::fs::try_exists("env.js").await? {
    tokio::fs::copy("node_modules/@lm_fe/scripts/assets/env.js", "env.js").await?;
  }

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
#[test]
fn test_simple_encrypt_str() {
  assert_eq!(simple_encrypt_str("123"), "-50@@68@@186");
}

pub async fn pre_work(dev_mod: bool) -> MyResult<(HashMap<String, String>, CheckVersion)> {
  let mut env_m = dot_env_to_map_new().await?;
  let mut check_v = CheckVersion::new("public", "dist").await;

  if env_m.contains_key("HOST_URL") {
    let host_url = env_m.remove("HOST_URL").unwrap();

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
