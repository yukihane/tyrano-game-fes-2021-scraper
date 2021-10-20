#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod init;

use anyhow::Result;
use init::fetch_data;

fn main() -> Result<()> {
  fetch_data()?;

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
