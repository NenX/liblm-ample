use flate2::{write::GzEncoder, Compression};
use fs_extra::dir::CopyOptions;

use std::path::Path;
use tokio::{fs, time::Instant};

use crate::util::{CheckVersion, MyResult, dot_env_to_map, format_date_time_underscore, run_command_spawn_envs};



const PACK_DIR: &str = "lm_packet";
#[cfg(windows)]
pub const BUILD_CMD: &str =
    r" node .\node_modules\webpack\bin\webpack.js --config webpack.prod.config.js";
#[cfg(not(windows))]
pub const BUILD_CMD: &str =
    r" node ./node_modules/webpack/bin/webpack.js --config webpack.prod.config.js";

pub async fn do_build() -> MyResult<()> {
    let dis_path = Path::new("dist");
    if !dis_path.is_dir() {
        fs::create_dir(&dis_path).await?
    }
    let mut env_m = dot_env_to_map().await?;
    let mut v = CheckVersion::from_file("check_version").await?;

    env_m.insert("check_version".to_string(), v.next().to_string());
    println!("开始构建: {:?}", env_m);

    std::fs::remove_dir_all("dist").expect("Failed to delete dist");
    std::fs::create_dir_all("dist").expect("Failed to create dist");
    // let mut del_c = run_command_spawn(r"del /Q /S dist\img dist\js dist\css").await?;
    // if !del_c.wait().await?.success() {
    //     return my_err_msg!(;"删除出错！");
    // }
    mov_the_fucking_things().expect("Failed to move files");
    // let mut xcopy_c = run_command_spawn(r"xcopy.exe public\* dist /S /D /I").await?;
    // if !xcopy_c.wait().await?.success() {
    //     return my_err_msg!(;"复制出错！");
    // }

    let mut build_task = run_command_spawn_envs(
        BUILD_CMD,
        // "dir",
        env_m.iter().map(|a| (a.0.as_str(), a.1.as_str())),
    )
    .await?;

    let start = Instant::now();

    let name = env_m.get("APP_NAME").cloned().unwrap_or("".to_string());

    let gz_name = format!(
        r"{}/{}_{}.tar.gz",
        PACK_DIR,
        name,
        format_date_time_underscore()
    );

    if !build_task.wait().await?.success() {
        return Err("haha".into());
    }
    v.to_file("dist").await?;
    println!("耗时 {:?}", start.elapsed());

    let start = Instant::now();

    println!("开始压缩");

    compress_dist(&gz_name).await?;
    println!("操作成功！{:?}", start.elapsed());
    // run_command_spawn(&format!("explorer {PACK_DIR}")).await?;

    Ok(())
}

pub async fn compress_dist(name: &str) -> Result<(), std::io::Error> {
    let dir_path = Path::new(PACK_DIR);
    if !dir_path.is_dir() {
        fs::create_dir_all(PACK_DIR).await?;
    }

    #[cfg(target_os = "windows")]
    {
        let tar_gz = std::fs::File::create(name)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(".", "dist")?;
        tar.finish()?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let tar_gz = std::fs::File::create(name)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);
        tar.append_dir_all(".", "dist")?;
        tar.finish()?;
    }

    Ok(())
}
fn mov_the_fucking_things() -> fs_extra::error::Result<()> {
    let target = "dist".to_string();
    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    options.skip_exist = true;
    // let from_paths = vec![
    //     "public/img"
    // ];
    // fs_extra::copy_items(&from_paths, target, &options)?;
    fs_extra::dir::copy("public","dist", &options)?;
    Ok(())
}
