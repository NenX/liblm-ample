use std::path::Path;

use tokio::fs;

use crate::util::error::{MyError, MyResult};

pub struct CheckVersion<'a> {
  pub n: u32,
  pub path: &'a Path,
}

impl<'a> CheckVersion<'a> {
  pub async fn from_file<T: AsRef<Path> + ?Sized>(file: &'a T) -> MyResult<Self> {
    let file = file.as_ref();
    if !file.exists() {
      fs::write(file, "0").await?;
    }

    let f = fs::read_to_string(file).await?;
    let n = f.trim().parse::<u32>()?;
    Ok(Self { n, path: file })
  }
  pub async fn to_file<T: AsRef<Path> + ?Sized>(&self, target: &T) -> MyResult<()> {
    let target = target.as_ref();
    fs::write(self.path, self.n.to_string()).await?;
    if !target.exists() {
      fs::create_dir_all(target).await?;
    }
    fs::write(target.join(self.path), self.n.to_string())
      .await?;

    Ok(())
  }
  pub fn next(&mut self) -> u32 {
    self.n += 1;
    self.n
  }
}
