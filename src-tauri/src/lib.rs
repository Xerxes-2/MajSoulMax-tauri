mod game;
use game::start_game;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![start_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
