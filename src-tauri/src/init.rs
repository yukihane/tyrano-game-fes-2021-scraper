use anyhow::Result;
use std::fs;
use std::path::Path;

pub fn fetch_data() -> Result<()> {
  let data_dir = Path::new("data");
  let data_file = data_dir.join("data.json");

  if data_file.exists() {
    return Ok(());
  }

  fs::create_dir_all(data_dir)?;

  Ok(())
}
