#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::fs;

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

fn handle_stock_creation(stockString: &str) -> String {
    let e = stock::entity::entity_from_string(stockString);
    println!("Yo {}", e.name);
    stock::entity::save(&e);
    format!("{}", true)
}

fn handle_stock_load() -> String {
    let paths = fs::read_dir("data/stock/").unwrap();
    for path in paths {
        let string = path.unwrap().file_name().into_string();
        //let file = file_manager::read_entity_from_path(&string.unwrap());
        //println!("Yo {}", &string.unwrap());
        format!("{}", &string.unwrap());
    }
    //format!("{}", string.unwrap())
    format!("{}", true)
}

fn main() {
    file_manager::create_dir_if_not_exist("data/stock/");
    file_manager::create_dir_if_not_exist("data/json/");
    file_manager::create_dir_if_not_exist("data/img/");
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![create_stock_entity])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
