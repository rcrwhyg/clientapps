// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;

fn main() -> Result<()> {
    hackernews_lib::app()?
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
