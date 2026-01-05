use crate::util::{
  MyResult, dot_env_to_map, dot_env_to_map_new, format_date_time_underscore, run_command_spawn_envs,
};

pub async fn do_start() -> MyResult<()> {
  let mut env_m = dot_env_to_map_new().await?;

  env_m.insert("ENVIRONMENT_MODE".to_string(), "development".to_string());
  env_m.insert("LM_BUILD_AT".to_string(), format_date_time_underscore());

  println!("开始运行: {:?}", env_m);

  let mut start_task = run_command_spawn_envs(
    "rspack serve",
    // "dir",
    env_m.iter().map(|a| (a.0.as_str(), a.1.as_str())),
  )
  .await?;

  if !start_task.wait().await?.success() {
    return Err("启动失败！".into());
  }

  Ok(())
}
