#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod stock;
mod file_manager;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn create_stock_entity(stockString: &str) -> String {
    handle_stock_creation(stockString);
    format!("{}", true)
}

fn handle_stock_creation(stockString: &str){
    let e = stock::entity::entity_from_string(stockString);
    println!("Yo {}", e.name);
    stock::entity::save(&e);
    println!("Yo2");
}

fn main() {
    file_manager::create_dir_if_not_exist("/data/stock/");
    file_manager::create_dir_if_not_exist("/data/json/");
    file_manager::create_dir_if_not_exist("/data/img/");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![create_stock_entity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
