use lyremember_backend::models::*;
use lyremember_backend::services::*;
use lyremember_backend::Result;
use rusqlite::Connection;
use std::sync::Mutex;
use tauri::State;

/// Shared database connection state
pub struct DbState(pub Mutex<Connection>);

// ==================== AUTH COMMANDS ====================

#[tauri::command]
pub async fn cmd_register(
    username: String,
    email: String,
    password: String,
    state: State<'_, DbState>,
) -> Result<User, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let data = RegisterData {
        username,
        email,
        password,
        genius_token: None,
    };
    
    auth::register(&conn, data)
        .map_err(|e| format!("Registration failed: {}", e))
}

#[tauri::command]
pub async fn cmd_login(
    username: String,
    password: String,
    state: State<'_, DbState>,
) -> Result<String, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let data = LoginData { username, password };
    
    auth::login(&conn, data)
        .map_err(|e| format!("Login failed: {}", e))
}

#[tauri::command]
pub async fn cmd_verify_token(token: String) -> Result<User, String> {
    auth::verify_token(&token)
        .map_err(|e| format!("Token verification failed: {}", e))
}

#[tauri::command]
pub async fn cmd_get_user(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<User, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    auth::get_user_by_id(&conn, &user_id)
        .map_err(|e| format!("Failed to get user: {}", e))
}

// ==================== SONGS COMMANDS ====================

#[tauri::command]
pub async fn cmd_create_song(
    title: String,
    artist: String,
    language: String,
    lyrics: Vec<String>,
    auto_translate: Option<bool>,
    state: State<'_, DbState>,
) -> Result<Song, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let data = CreateSongData {
        title,
        artist,
        language,
        lyrics,
        auto_translate,
    };
    
    songs::create_song(&conn, data)
        .map_err(|e| format!("Failed to create song: {}", e))
}

#[tauri::command]
pub async fn cmd_get_songs(state: State<'_, DbState>) -> Result<Vec<Song>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    songs::get_all_songs(&conn)
        .map_err(|e| format!("Failed to get songs: {}", e))
}

#[tauri::command]
pub async fn cmd_get_song(
    song_id: String,
    state: State<'_, DbState>,
) -> Result<Song, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    songs::get_song(&conn, &song_id)
        .map_err(|e| format!("Failed to get song: {}", e))
}

#[tauri::command]
pub async fn cmd_get_user_songs(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<Vec<Song>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    songs::get_user_songs(&conn, &user_id)
        .map_err(|e| format!("Failed to get user songs: {}", e))
}

#[tauri::command]
pub async fn cmd_add_to_repertoire(
    user_id: String,
    song_id: String,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    songs::add_to_user_repertoire(&conn, &user_id, &song_id)
        .map_err(|e| format!("Failed to add to repertoire: {}", e))
}

#[tauri::command]
pub async fn cmd_update_song(
    song_id: String,
    title: Option<String>,
    artist: Option<String>,
    lyrics: Option<Vec<String>>,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let data = UpdateSongData {
        title,
        artist,
        lyrics,
    };
    
    songs::update_song(&conn, &song_id, data)
        .map_err(|e| format!("Failed to update song: {}", e))
}

#[tauri::command]
pub async fn cmd_delete_song(
    song_id: String,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    songs::delete_song(&conn, &song_id)
        .map_err(|e| format!("Failed to delete song: {}", e))
}

// ==================== PRACTICE COMMANDS ====================

#[tauri::command]
pub async fn cmd_create_practice_session(
    user_id: String,
    song_id: String,
    mode: String,
    score: f64,
    lines_practiced: i32,
    lines_correct: i32,
    duration_seconds: i32,
    state: State<'_, DbState>,
) -> Result<PracticeSession, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    let data = CreateSessionData {
        user_id,
        song_id,
        mode,
        score,
        lines_practiced,
        lines_correct,
        duration_seconds,
    };
    
    practice::create_session(&conn, data)
        .map_err(|e| format!("Failed to create practice session: {}", e))
}

#[tauri::command]
pub async fn cmd_get_user_sessions(
    user_id: String,
    limit: Option<i32>,
    state: State<'_, DbState>,
) -> Result<Vec<PracticeSession>, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    practice::get_user_sessions(&conn, &user_id, limit)
        .map_err(|e| format!("Failed to get user sessions: {}", e))
}

#[tauri::command]
pub async fn cmd_get_user_stats(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<UserStats, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    practice::get_user_stats(&conn, &user_id)
        .map_err(|e| format!("Failed to get user stats: {}", e))
}

#[tauri::command]
pub async fn cmd_get_song_mastery(
    user_id: String,
    song_id: String,
    state: State<'_, DbState>,
) -> Result<SongMastery, String> {
    let conn = state.0.lock().map_err(|e| e.to_string())?;
    
    practice::get_song_mastery(&conn, &user_id, &song_id)
        .map_err(|e| format!("Failed to get song mastery: {}", e))
}

// ==================== UTILITY COMMANDS ====================

#[tauri::command]
pub async fn cmd_translate_text(
    text: String,
    source_lang: String,
    target_lang: String,
) -> Result<Vec<String>, String> {
    let lines = vec![text];
    tokio::task::spawn_blocking(move || {
        translation::translate_text(lines, &source_lang, &target_lang)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e| format!("Translation failed: {}", e))
}

#[tauri::command]
pub async fn cmd_generate_phonetic(
    text: Vec<String>,
    language: String,
) -> Result<Vec<String>, String> {
    tokio::task::spawn_blocking(move || {
        phonetic::generate_phonetic(text, &language)
    })
    .await
    .map_err(|e| format!("Task join error: {}", e))?
    .map_err(|e| format!("Phonetic generation failed: {}", e))
}

#[tauri::command]
pub async fn cmd_health_check() -> Result<String, String> {
    Ok("Backend is healthy!".to_string())
}
