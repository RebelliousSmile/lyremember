//! Example usage of LyRemember backend

use lyremember_backend::{
    db::init_database,
    services::{
        auth::{register, login, RegisterData, LoginCredentials},
        songs::{create_song, get_user_songs, add_to_user_repertoire, CreateSongData},
        practice::{create_session, get_user_stats, CreateSessionData},
    },
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎵 LyRemember Backend Example\n");

    // Initialize database
    let conn = init_database("example.db")?;
    println!("✅ Database initialized");

    // Register a user
    let register_data = RegisterData {
        username: "musiclover".to_string(),
        email: "lover@music.com".to_string(),
        password: "securepassword123".to_string(),
    };
    
    let user = register(&conn, register_data)?;
    println!("✅ User registered: {}", user.username);

    // Login
    let credentials = LoginCredentials {
        username: "musiclover".to_string(),
        password: "securepassword123".to_string(),
    };
    
    let (logged_user, token) = login(&conn, credentials)?;
    println!("✅ User logged in, JWT: {}...", &token[..20]);

    // Create a song (English)
    let song_data = CreateSongData {
        title: "Imagine".to_string(),
        artist: "John Lennon".to_string(),
        language: "en".to_string(),
        lyrics: vec![
            "Imagine there's no heaven".to_string(),
            "It's easy if you try".to_string(),
            "No hell below us".to_string(),
            "Above us only sky".to_string(),
        ],
    };
    
    let song = create_song(&conn, song_data)?;
    println!("\n✅ Song created: {} by {}", song.title, song.artist);
    
    if let Some(phonetic) = &song.phonetic_lyrics {
        println!("📝 Phonetic generated: {} lines", phonetic.len());
    }

    // Add song to user's repertoire
    add_to_user_repertoire(&conn, &logged_user.id, &song.id)?;
    println!("✅ Song added to user's repertoire");

    // Get user's songs
    let user_songs = get_user_songs(&conn, &logged_user.id)?;
    println!("✅ User has {} songs in repertoire", user_songs.len());

    // Create a practice session
    let session_data = CreateSessionData {
        user_id: logged_user.id.clone(),
        song_id: song.id.clone(),
        mode: "karaoke".to_string(),
        score: 85.5,
        lines_practiced: 4,
        lines_correct: 3,
        duration_seconds: 120,
    };
    
    let session = create_session(&conn, session_data)?;
    println!("\n✅ Practice session created:");
    println!("   Mode: {}", session.mode);
    println!("   Score: {:.1}%", session.score);
    println!("   Duration: {}s", session.duration_seconds);

    // Get user stats
    let stats = get_user_stats(&conn, &logged_user.id)?;
    println!("\n📊 User Statistics:");
    println!("   Total sessions: {}", stats.total_sessions);
    println!("   Total practice time: {}s", stats.total_practice_time);
    println!("   Average score: {:.1}%", stats.average_score);

    println!("\n🎉 Example completed successfully!");
    
    Ok(())
}
