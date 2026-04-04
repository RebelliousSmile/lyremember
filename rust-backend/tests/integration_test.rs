//! Integration test: full user flow
//! register → login → create song → add to repertoire → practice → stats

use lyremember_backend::db::init_database;
use lyremember_backend::models::*;
use lyremember_backend::services::*;
use tempfile::NamedTempFile;

#[test]
fn test_full_user_flow() {
    let temp_file = NamedTempFile::new().unwrap();
    let conn = init_database(temp_file.path()).unwrap();

    // 1. Register
    let user = auth::register(
        &conn,
        RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        },
    )
    .unwrap();
    assert_eq!(user.username, "testuser");

    // 2. Login
    let (logged_in_user, token) = auth::login(
        &conn,
        LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        },
    )
    .unwrap();
    assert_eq!(logged_in_user.id, user.id);
    assert!(!token.is_empty());

    // 3. Verify token → returns user_id
    let verified_user_id = auth::verify_token(&token).unwrap();
    assert_eq!(verified_user_id, user.id);

    // 4. Create song (phonetic auto-generated via pure-Rust)
    let song = songs::create_song(
        &conn,
        CreateSongData {
            title: "Sakura".to_string(),
            artist: "Test Artist".to_string(),
            language: "jp".to_string(),
            lyrics: vec![
                "さくら さくら".to_string(),
                "やよいの そらは".to_string(),
            ],
        },
    )
    .unwrap();
    assert_eq!(song.title, "Sakura");
    assert!(song.phonetic_lyrics.is_some(), "Phonetic should be auto-generated for JP");

    // 5. Add song to user's repertoire
    songs::add_to_user_repertoire(&conn, &user.id, &song.id).unwrap();

    let user_songs = songs::get_user_songs(&conn, &user.id).unwrap();
    assert_eq!(user_songs.len(), 1);
    assert_eq!(user_songs[0].id, song.id);

    // 6. Create practice session
    let session = practice::create_session(
        &conn,
        CreateSessionData {
            user_id: user.id.clone(),
            song_id: song.id.clone(),
            mode: "karaoke".to_string(),
            score: 92.5,
            lines_practiced: 2,
            lines_correct: 2,
            duration_seconds: 45,
        },
    )
    .unwrap();
    assert_eq!(session.score, 92.5);

    // 7. Get user sessions (with and without limit)
    let sessions = practice::get_user_sessions(&conn, &user.id, None).unwrap();
    assert_eq!(sessions.len(), 1);

    let limited = practice::get_user_sessions(&conn, &user.id, Some(10)).unwrap();
    assert_eq!(limited.len(), 1);

    // 8. Get user stats
    let stats = practice::get_user_stats(&conn, &user.id).unwrap();
    assert_eq!(stats.total_sessions, 1);
    assert_eq!(stats.total_practice_time, 45);
    assert!((stats.average_score - 92.5).abs() < 0.01);

    // 9. Get song mastery
    let mastery = practice::get_song_mastery(&conn, &user.id, &song.id).unwrap();
    assert!((mastery - 92.5).abs() < 0.01);
}

#[test]
fn test_db_migration_idempotent() {
    let temp_file = NamedTempFile::new().unwrap();

    // Init twice — should not fail
    let _conn1 = init_database(temp_file.path()).unwrap();
    let conn2 = init_database(temp_file.path()).unwrap();

    // Schema version should be set
    let version: i32 = conn2
        .query_row("SELECT MAX(version) FROM schema_version", [], |row| row.get(0))
        .unwrap();
    assert!(version >= 1);
}
