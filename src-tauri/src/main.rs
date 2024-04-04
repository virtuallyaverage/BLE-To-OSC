// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::vec;

//import modules
mod bluetooth;


//setup bluetooth functions
#[tauri::command]
async fn list_bt_devices(_app: tauri::AppHandle, _window: tauri::Window) -> Result<Vec<String>, String> {
  let device_list:Vec<String> = bluetooth::list_bt_devices().await?;
  println!("{:#?}", device_list);
  return Ok(device_list)

}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, list_bt_devices])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
