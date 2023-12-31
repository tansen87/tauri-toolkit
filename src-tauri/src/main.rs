#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]


use dlib::data4mysql;
use dlib::dataprocess;
use dlib::excel2csv;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            data4mysql::download,
            dataprocess::pivot,
            dataprocess::unique,
            dataprocess::concat,
            excel2csv::etoc
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
