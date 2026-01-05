use std::collections::HashMap;

use chrono::Local;

use crate::util::{error::MyResult, run_command};

pub fn format_date_time() -> String {
  let now = Local::now();

  now.format("%Y-%m-%d %H:%M:%S").to_string()
}
pub fn format_date_time_underscore() -> String {
  let now = Local::now();

  now.format("%y_%m%d_%H%M").to_string()
}

pub async fn dot_env_to_map() -> MyResult<HashMap<String, String>> {
  use tokio::io::AsyncBufReadExt;
  let mut m = HashMap::new();
  let file = tokio::fs::File::open(".env").await;
  let file = match file {
    Ok(f) => f,
    Err(e) => {
      eprintln!("Failed to open .env file: {}, try to open .env.sh", e);
      tokio::fs::File::open(".env.sh").await?
    }
  };
  let mut reader = tokio::io::BufReader::new(file);
  let mut s = String::new();
  // 逐行异步读取文件
  while reader.read_line(&mut s).await? != 0 {
    if !s.contains('=') || s.starts_with('#') {
      s.clear();
      continue;
    };
    let mut split_line = s.split('=');

    let mut format_str = || -> String {
      let q = split_line.next().unwrap_or("");
      q.replace(['\'', '"'], "")
        .replace("export", "")
        .trim()
        .to_string()
    };

    m.insert(format_str(), format_str());
    s.clear();
  }

  Ok(m)
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
