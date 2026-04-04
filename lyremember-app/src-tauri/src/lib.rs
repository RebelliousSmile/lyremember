mod commands;

use commands::*;
use lyremember_backend::db::init_database;
use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Get app data directory
            let app_data_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            // Create directory if it doesn't exist
            std::fs::create_dir_all(&app_data_dir)
                .expect("Failed to create app data directory");
            
            // Initialize database in app data directory
            let db_path = app_data_dir.join("lyremember.db");
            let db_path_str = db_path.to_str().expect("Invalid database path");
            
            println!("Initializing database at: {}", db_path_str);
            
            let conn = init_database(db_path_str)
                .expect("Failed to initialize database");
            
            // Store database connection in app state
            app.manage(DbState(Mutex::new(conn)));
            
            println!("Database initialized successfully!");
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Auth commands
            cmd_register,
            cmd_login,
            cmd_login_as_guest,
            cmd_verify_token,
            cmd_get_user,
            // Songs commands
            cmd_create_song,
            cmd_get_songs,
            cmd_get_song,
            cmd_get_user_songs,
            cmd_add_to_repertoire,
            cmd_update_song,
            cmd_delete_song,
            // Practice commands
            cmd_create_practice_session,
            cmd_get_user_sessions,
            cmd_get_user_stats,
            cmd_get_song_mastery,
            // Utility commands
            cmd_translate_text,
            cmd_generate_phonetic,
            cmd_health_check,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
