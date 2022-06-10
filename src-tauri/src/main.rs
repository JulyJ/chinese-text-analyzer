#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod utils;
mod analyzer;

use std::fs;

use analyzer::{Analyzer, AnalyzedCounterOutput};

#[tauri::command]
async fn analyze_file(analyzer: tauri::State<'_, Analyzer>, file_path: String) -> Result<AnalyzedCounterOutput, bool> {
    println!("{}", file_path);
    
    let text = fs::read_to_string(file_path).unwrap();
    
    Ok(analyzer.analyze(&text))
}

fn main() {
    let submenu = tauri::Menu::new()
        .add_native_item(tauri::MenuItem::Copy)
        .add_native_item(tauri::MenuItem::Paste);
    
    let menu = tauri::Menu::new()
        .add_submenu(tauri::Submenu::new("Chinese Text Analyzer", submenu));
    
    tauri::Builder::default()
        .menu(menu)
        .manage(Analyzer::new())
        .invoke_handler(tauri::generate_handler![
            analyze_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
