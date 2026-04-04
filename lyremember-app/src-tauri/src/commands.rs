use lyremember_backend::models::*;
use lyremember_backend::rusqlite::Connection;
use lyremember_backend::services::*;
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

/// Shared database connection state
pub struct DbState(pub Mutex<Connection>);

/// Login response with user and token
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

/// Helper to lock the database connection
fn lock_db<'a>(state: &'a State<'a, DbState>) -> Result<std::sync::MutexGuard<'a, Connection>, String> {
    state.0.lock().map_err(|e: std::sync::PoisonError<_>| e.to_string())
}

// ==================== AUTH COMMANDS ====================

#[tauri::command]
pub fn cmd_register(
    username: String,
    email: String,
    password: String,
    state: State<'_, DbState>,
) -> Result<User, String> {
    let conn = lock_db(&state)?;
    let data = RegisterData { username, email, password };
    auth::register(&conn, data).map_err(|e| format!("Registration failed: {}", e))
}

#[tauri::command]
pub fn cmd_login(
    username: String,
    password: String,
    state: State<'_, DbState>,
) -> Result<String, String> {
    let conn = lock_db(&state)?;
    let credentials = LoginCredentials { username, password };
    let (_user, token) = auth::login(&conn, credentials)
        .map_err(|e| format!("Login failed: {}", e))?;
    Ok(token)
}

#[tauri::command]
pub fn cmd_verify_token(token: String) -> Result<String, String> {
    auth::verify_token(&token).map_err(|e| format!("Token verification failed: {}", e))
}

#[tauri::command]
pub fn cmd_get_user(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<User, String> {
    let conn = lock_db(&state)?;
    auth::get_user_by_id(&conn, &user_id).map_err(|e| format!("Failed to get user: {}", e))
}

#[tauri::command]
pub fn cmd_login_as_guest(
    state: State<'_, DbState>,
) -> Result<LoginResponse, String> {
    let conn = lock_db(&state)?;
    let (user, token) = auth::login_as_guest(&conn)
        .map_err(|e| format!("Guest login failed: {}", e))?;
    Ok(LoginResponse { user, token })
}

// ==================== SONGS COMMANDS ====================

#[tauri::command]
pub fn cmd_create_song(
    title: String,
    artist: String,
    language: String,
    lyrics: Vec<String>,
    state: State<'_, DbState>,
) -> Result<Song, String> {
    let conn = lock_db(&state)?;
    let data = CreateSongData { title, artist, language, lyrics };
    songs::create_song(&conn, data).map_err(|e| format!("Failed to create song: {}", e))
}

#[tauri::command]
pub fn cmd_get_songs(state: State<'_, DbState>) -> Result<Vec<Song>, String> {
    let conn = lock_db(&state)?;
    songs::get_all_songs(&conn).map_err(|e| format!("Failed to get songs: {}", e))
}

#[tauri::command]
pub fn cmd_get_song(
    song_id: String,
    state: State<'_, DbState>,
) -> Result<Song, String> {
    let conn = lock_db(&state)?;
    songs::get_song(&conn, &song_id).map_err(|e| format!("Failed to get song: {}", e))
}

#[tauri::command]
pub fn cmd_get_user_songs(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<Vec<Song>, String> {
    let conn = lock_db(&state)?;
    songs::get_user_songs(&conn, &user_id).map_err(|e| format!("Failed to get user songs: {}", e))
}

#[tauri::command]
pub fn cmd_add_to_repertoire(
    user_id: String,
    song_id: String,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = lock_db(&state)?;
    songs::add_to_user_repertoire(&conn, &user_id, &song_id)
        .map_err(|e| format!("Failed to add to repertoire: {}", e))
}

#[tauri::command]
pub fn cmd_update_song(
    song_id: String,
    title: Option<String>,
    artist: Option<String>,
    language: Option<String>,
    lyrics: Option<Vec<String>>,
    phonetic_lyrics: Option<Vec<String>>,
    translations: Option<std::collections::HashMap<String, Vec<String>>>,
    state: State<'_, DbState>,
) -> Result<Song, String> {
    let conn = lock_db(&state)?;
    let data = UpdateSongData {
        title,
        artist,
        language,
        lyrics,
        phonetic_lyrics,
        translations,
    };
    songs::update_song(&conn, &song_id, data).map_err(|e| format!("Failed to update song: {}", e))
}

#[tauri::command]
pub fn cmd_delete_song(
    song_id: String,
    state: State<'_, DbState>,
) -> Result<(), String> {
    let conn = lock_db(&state)?;
    songs::delete_song(&conn, &song_id).map_err(|e| format!("Failed to delete song: {}", e))
}

// ==================== PRACTICE COMMANDS ====================

#[tauri::command]
pub fn cmd_create_practice_session(
    user_id: String,
    song_id: String,
    mode: String,
    score: f64,
    lines_practiced: i32,
    lines_correct: i32,
    duration_seconds: i32,
    state: State<'_, DbState>,
) -> Result<PracticeSession, String> {
    let conn = lock_db(&state)?;
    let data = CreateSessionData {
        user_id, song_id, mode, score,
        lines_practiced, lines_correct, duration_seconds,
    };
    practice::create_session(&conn, data)
        .map_err(|e| format!("Failed to create practice session: {}", e))
}

#[tauri::command]
pub fn cmd_get_user_sessions(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<Vec<PracticeSession>, String> {
    let conn = lock_db(&state)?;
    practice::get_user_sessions(&conn, &user_id)
        .map_err(|e| format!("Failed to get user sessions: {}", e))
}

#[tauri::command]
pub fn cmd_get_user_stats(
    user_id: String,
    state: State<'_, DbState>,
) -> Result<practice::UserStats, String> {
    let conn = lock_db(&state)?;
    practice::get_user_stats(&conn, &user_id)
        .map_err(|e| format!("Failed to get user stats: {}", e))
}

#[tauri::command]
pub fn cmd_get_song_mastery(
    user_id: String,
    song_id: String,
    state: State<'_, DbState>,
) -> Result<f64, String> {
    let conn = lock_db(&state)?;
    practice::get_song_mastery(&conn, &user_id, &song_id)
        .map_err(|e| format!("Failed to get song mastery: {}", e))
}

// ==================== UTILITY COMMANDS ====================

#[tauri::command]
pub fn cmd_translate_text(
    text: Vec<String>,
    source_lang: String,
    target_lang: String,
) -> Result<Vec<String>, String> {
    translation::translate_text(text, &source_lang, &target_lang)
        .map_err(|e| format!("Translation failed: {}", e))
}

#[tauri::command]
pub fn cmd_generate_phonetic(
    text: Vec<String>,
    language: String,
) -> Result<Vec<String>, String> {
    phonetic::generate_phonetic(&text, &language)
        .map_err(|e| format!("Phonetic generation failed: {}", e))
}

#[tauri::command]
pub fn cmd_health_check() -> Result<String, String> {
    Ok("Backend is healthy!".to_string())
}
