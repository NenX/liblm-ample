use std::path::Path;

use tokio::fs;

use crate::util::error::MyResult;
const FILE_NAME: &str = "check_version";

pub struct CheckVersion {
  pub n: u32,
  src_dir: String,
  dst_dir: String,
}
impl CheckVersion {
  pub async fn new(src: &str, dst: &str) -> Self {
    let mut s = Self {
      n: 0,
      src_dir: src.to_string(),
      dst_dir: dst.to_string(),
    };
    s.read().await.expect("read version");
    s
  }
  async fn read(&mut self) -> MyResult<&Self> {
    let file: &Path = self.src_dir.as_ref();
    let file = file.join(FILE_NAME);
    if !file.exists() {
      fs::write(&file, "0").await?;
      return Ok(self);
    }

    let f = fs::read_to_string(file).await?;
    self.n = f.trim().parse::<u32>()?;
    Ok(self)
  }
  pub async fn write(&self) -> MyResult<()> {
    let dst_path: &Path = self.dst_dir.as_ref();
    let src_path: &Path = self.src_dir.as_ref();
    let dst = dst_path.join(FILE_NAME);
    let src = src_path.join(FILE_NAME);

    if !dst_path.exists() {
      fs::create_dir_all(&dst_path).await?;
    }

    fs::write(dst, self.n.to_string()).await?;
    fs::write(src, self.n.to_string()).await?;

    Ok(())
  }
  pub fn next(&mut self) -> u32 {
    self.n += 1;
    self.n
  }
}
