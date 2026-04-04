//! Songs management service

pub use crate::models::{Song, CreateSongData, UpdateSongData};

use crate::{Error, Result};
use crate::services::{phonetic, translation};
use rusqlite::Connection;
use std::collections::HashMap;

/// Create a new song with automatic translation and phonetic generation
pub fn create_song(conn: &Connection, data: CreateSongData) -> Result<Song> {
    let mut song = Song::new(data.title, data.artist, data.language, data.lyrics.clone());
    
    // Auto-generate phonetic if language supports it
    if matches!(song.language.as_str(), "jp" | "kr" | "fr" | "en") {
        match phonetic::generate_phonetic(&data.lyrics, &song.language) {
            Ok(phonetic_lyrics) => {
                song.phonetic_lyrics = Some(phonetic_lyrics);
            }
            Err(e) => {
                eprintln!("Warning: Failed to generate phonetic: {}", e);
                // Continue without phonetics
            }
        }
    }
    
    // Auto-translate to English if original language is not English
    if song.language != "en" {
        match translation::translate_text(data.lyrics.clone(), &song.language, "en") {
            Ok(translated) => {
                let mut translations = HashMap::new();
                translations.insert("en".to_string(), translated);
                song.translations = Some(translations);
            }
            Err(e) => {
                eprintln!("Warning: Failed to translate: {}", e);
                // Continue without translation
            }
        }
    }
    
    // Insert into database
    save_song(conn, &song)?;
    
    Ok(song)
}

/// Save song to database
fn save_song(conn: &Connection, song: &Song) -> Result<()> {
    let lyrics_json = serde_json::to_string(&song.lyrics)?;
    let phonetic_json = song.phonetic_lyrics.as_ref()
        .map(|p| serde_json::to_string(p))
        .transpose()?;
    let translations_json = song.translations.as_ref()
        .map(|t| serde_json::to_string(t))
        .transpose()?;
    
    conn.execute(
        "INSERT INTO songs 
         (id, title, artist, language, lyrics, phonetic_lyrics, translations, 
          genius_id, genius_url, created_at, updated_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        rusqlite::params![
            &song.id,
            &song.title,
            &song.artist,
            &song.language,
            &lyrics_json,
            &phonetic_json,
            &translations_json,
            &song.genius_id,
            &song.genius_url,
            &song.created_at,
            &song.updated_at,
        ],
    )?;
    
    Ok(())
}

/// Get song by ID
pub fn get_song(conn: &Connection, song_id: &str) -> Result<Song> {
    let mut stmt = conn.prepare(
        "SELECT id, title, artist, language, lyrics, phonetic_lyrics, translations,
                genius_id, genius_url, created_at, updated_at
         FROM songs WHERE id = ?1"
    )?;
    
    let song = stmt.query_row([song_id], |row| {
        let lyrics_json: String = row.get(4)?;
        let phonetic_json: Option<String> = row.get(5)?;
        let translations_json: Option<String> = row.get(6)?;
        
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            language: row.get(3)?,
            lyrics: serde_json::from_str(&lyrics_json).unwrap_or_default(),
            phonetic_lyrics: phonetic_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            translations: translations_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            genius_id: row.get(7)?,
            genius_url: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }).map_err(|_| Error::NotFound("Song not found".to_string()))?;
    
    Ok(song)
}

/// Get all songs
pub fn get_all_songs(conn: &Connection) -> Result<Vec<Song>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, artist, language, lyrics, phonetic_lyrics, translations,
                genius_id, genius_url, created_at, updated_at
         FROM songs
         ORDER BY created_at DESC"
    )?;
    
    let songs = stmt.query_map([], |row| {
        let lyrics_json: String = row.get(4)?;
        let phonetic_json: Option<String> = row.get(5)?;
        let translations_json: Option<String> = row.get(6)?;
        
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            language: row.get(3)?,
            lyrics: serde_json::from_str(&lyrics_json).unwrap_or_default(),
            phonetic_lyrics: phonetic_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            translations: translations_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            genius_id: row.get(7)?,
            genius_url: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?.collect::<rusqlite::Result<Vec<_>>>()?;
    
    Ok(songs)
}

/// Get songs for a specific user
pub fn get_user_songs(conn: &Connection, user_id: &str) -> Result<Vec<Song>> {
    let mut stmt = conn.prepare(
        "SELECT s.id, s.title, s.artist, s.language, s.lyrics, s.phonetic_lyrics, 
                s.translations, s.genius_id, s.genius_url, s.created_at, s.updated_at
         FROM songs s
         INNER JOIN user_songs us ON s.id = us.song_id
         WHERE us.user_id = ?1
         ORDER BY us.added_at DESC"
    )?;
    
    let songs = stmt.query_map([user_id], |row| {
        let lyrics_json: String = row.get(4)?;
        let phonetic_json: Option<String> = row.get(5)?;
        let translations_json: Option<String> = row.get(6)?;
        
        Ok(Song {
            id: row.get(0)?,
            title: row.get(1)?,
            artist: row.get(2)?,
            language: row.get(3)?,
            lyrics: serde_json::from_str(&lyrics_json).unwrap_or_default(),
            phonetic_lyrics: phonetic_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            translations: translations_json
                .and_then(|j| serde_json::from_str(&j).ok()),
            genius_id: row.get(7)?,
            genius_url: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    })?.collect::<rusqlite::Result<Vec<_>>>()?;
    
    Ok(songs)
}

/// Add song to user's repertoire
pub fn add_to_user_repertoire(conn: &Connection, user_id: &str, song_id: &str) -> Result<()> {
    let added_at = chrono::Utc::now().to_rfc3339();
    
    conn.execute(
        "INSERT OR IGNORE INTO user_songs (user_id, song_id, added_at)
         VALUES (?1, ?2, ?3)",
        rusqlite::params![user_id, song_id, added_at],
    )?;
    
    Ok(())
}

/// Update song
pub fn update_song(conn: &Connection, song_id: &str, data: UpdateSongData) -> Result<Song> {
    let mut song = get_song(conn, song_id)?;
    
    // Update fields if provided
    if let Some(title) = data.title {
        song.title = title;
    }
    if let Some(artist) = data.artist {
        song.artist = artist;
    }
    if let Some(language) = data.language {
        song.language = language;
    }
    if let Some(lyrics) = data.lyrics {
        song.lyrics = lyrics;
    }
    if let Some(phonetic) = data.phonetic_lyrics {
        song.phonetic_lyrics = Some(phonetic);
    }
    if let Some(translations) = data.translations {
        song.translations = Some(translations);
    }
    
    song.updated_at = chrono::Utc::now().to_rfc3339();
    
    // Update in database
    let lyrics_json = serde_json::to_string(&song.lyrics)?;
    let phonetic_json = song.phonetic_lyrics.as_ref()
        .map(|p| serde_json::to_string(p))
        .transpose()?;
    let translations_json = song.translations.as_ref()
        .map(|t| serde_json::to_string(t))
        .transpose()?;
    
    conn.execute(
        "UPDATE songs 
         SET title = ?1, artist = ?2, language = ?3, lyrics = ?4,
             phonetic_lyrics = ?5, translations = ?6, updated_at = ?7
         WHERE id = ?8",
        rusqlite::params![
            &song.title,
            &song.artist,
            &song.language,
            &lyrics_json,
            &phonetic_json,
            &translations_json,
            &song.updated_at,
            song_id,
        ],
    )?;
    
    Ok(song)
}

/// Delete song
pub fn delete_song(conn: &Connection, song_id: &str) -> Result<()> {
    let affected = conn.execute("DELETE FROM songs WHERE id = ?1", [song_id])?;
    
    if affected == 0 {
        return Err(Error::NotFound("Song not found".to_string()));
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use crate::services::auth::{register, RegisterData};
    use tempfile::NamedTempFile;

    /// Helper to create a fresh database connection for each test
    fn setup_db() -> (NamedTempFile, Connection) {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        (temp_file, conn)
    }

    /// Helper to create a song directly via save_song (bypasses translation/phonetic)
    fn insert_test_song(conn: &Connection, title: &str, artist: &str, language: &str) -> Song {
        let song = Song::new(
            title.to_string(),
            artist.to_string(),
            language.to_string(),
            vec!["Line 1".to_string(), "Line 2".to_string()],
        );
        save_song(conn, &song).unwrap();
        song
    }

    // ---- create_song ----

    #[test]
    fn test_create_and_get_song() {
        let (_tmp, conn) = setup_db();

        let data = CreateSongData {
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            language: "en".to_string(),
            lyrics: vec!["Line 1".to_string(), "Line 2".to_string()],
        };

        let song = create_song(&conn, data).unwrap();
        assert_eq!(song.title, "Test Song");
        assert_eq!(song.artist, "Test Artist");
        assert_eq!(song.language, "en");

        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.title, song.title);
        assert_eq!(retrieved.artist, song.artist);
        assert_eq!(retrieved.lyrics.len(), 2);
        assert_eq!(retrieved.lyrics[0], "Line 1");
        assert_eq!(retrieved.lyrics[1], "Line 2");
    }

    #[test]
    fn test_create_song_generates_unique_ids() {
        let (_tmp, conn) = setup_db();

        let song1 = insert_test_song(&conn, "Song A", "Artist A", "en");
        let song2 = insert_test_song(&conn, "Song B", "Artist B", "en");

        assert_ne!(song1.id, song2.id);
    }

    #[test]
    fn test_create_song_sets_timestamps() {
        let (_tmp, conn) = setup_db();
        let song = insert_test_song(&conn, "Song", "Artist", "en");

        assert!(!song.created_at.is_empty());
        assert!(!song.updated_at.is_empty());
        assert_eq!(song.created_at, song.updated_at);
    }

    #[test]
    fn test_create_song_defaults_optional_fields_to_none() {
        let (_tmp, conn) = setup_db();
        let song = insert_test_song(&conn, "Song", "Artist", "en");

        let retrieved = get_song(&conn, &song.id).unwrap();
        assert!(retrieved.genius_id.is_none());
        assert!(retrieved.genius_url.is_none());
    }

    // ---- get_song ----

    #[test]
    fn test_get_song_not_found() {
        let (_tmp, conn) = setup_db();

        let result = get_song(&conn, "nonexistent-uuid");
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Song not found"));
    }

    #[test]
    fn test_get_song_preserves_lyrics_order() {
        let (_tmp, conn) = setup_db();

        let lyrics = vec![
            "First".to_string(),
            "Second".to_string(),
            "Third".to_string(),
            "Fourth".to_string(),
        ];
        let song = Song::new(
            "Order Test".to_string(),
            "Artist".to_string(),
            "en".to_string(),
            lyrics.clone(),
        );
        save_song(&conn, &song).unwrap();

        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.lyrics, lyrics);
    }

    // ---- get_all_songs ----

    #[test]
    fn test_get_all_songs_empty() {
        let (_tmp, conn) = setup_db();
        let songs = get_all_songs(&conn).unwrap();
        assert!(songs.is_empty());
    }

    #[test]
    fn test_get_all_songs() {
        let (_tmp, conn) = setup_db();

        for i in 1..=3 {
            insert_test_song(&conn, &format!("Song {}", i), "Artist", "en");
        }

        let songs = get_all_songs(&conn).unwrap();
        assert_eq!(songs.len(), 3);
    }

    #[test]
    fn test_get_all_songs_ordered_by_created_at_desc() {
        let (_tmp, conn) = setup_db();

        // Insert songs with slightly different timestamps
        let s1 = insert_test_song(&conn, "First", "Artist", "en");
        let _s2 = insert_test_song(&conn, "Second", "Artist", "en");
        let s3 = insert_test_song(&conn, "Third", "Artist", "en");

        let songs = get_all_songs(&conn).unwrap();
        assert_eq!(songs.len(), 3);
        // Most recent first (DESC order)
        assert_eq!(songs[0].id, s3.id);
        assert_eq!(songs[2].id, s1.id);
    }

    // ---- get_user_songs ----

    #[test]
    fn test_get_user_songs_empty() {
        let (_tmp, conn) = setup_db();

        let user = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let songs = get_user_songs(&conn, &user.id).unwrap();
        assert!(songs.is_empty());
    }

    #[test]
    fn test_get_user_songs_returns_only_user_repertoire() {
        let (_tmp, conn) = setup_db();

        let user1 = register(&conn, RegisterData {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let user2 = register(&conn, RegisterData {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let song_a = insert_test_song(&conn, "Song A", "Artist", "en");
        let song_b = insert_test_song(&conn, "Song B", "Artist", "en");
        let song_c = insert_test_song(&conn, "Song C", "Artist", "en");

        add_to_user_repertoire(&conn, &user1.id, &song_a.id).unwrap();
        add_to_user_repertoire(&conn, &user1.id, &song_b.id).unwrap();
        add_to_user_repertoire(&conn, &user2.id, &song_c.id).unwrap();

        let user1_songs = get_user_songs(&conn, &user1.id).unwrap();
        assert_eq!(user1_songs.len(), 2);

        let user2_songs = get_user_songs(&conn, &user2.id).unwrap();
        assert_eq!(user2_songs.len(), 1);
        assert_eq!(user2_songs[0].id, song_c.id);
    }

    // ---- add_to_user_repertoire ----

    #[test]
    fn test_add_to_user_repertoire() {
        let (_tmp, conn) = setup_db();

        let user = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let song = insert_test_song(&conn, "Song", "Artist", "en");

        add_to_user_repertoire(&conn, &user.id, &song.id).unwrap();

        let songs = get_user_songs(&conn, &user.id).unwrap();
        assert_eq!(songs.len(), 1);
        assert_eq!(songs[0].id, song.id);
    }

    #[test]
    fn test_add_to_user_repertoire_duplicate_is_ignored() {
        let (_tmp, conn) = setup_db();

        let user = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let song = insert_test_song(&conn, "Song", "Artist", "en");

        // Add the same song twice
        add_to_user_repertoire(&conn, &user.id, &song.id).unwrap();
        add_to_user_repertoire(&conn, &user.id, &song.id).unwrap();

        let songs = get_user_songs(&conn, &user.id).unwrap();
        assert_eq!(songs.len(), 1);
    }

    // ---- update_song ----

    #[test]
    fn test_update_song_title() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Original Title", "Artist", "en");

        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: Some("New Title".to_string()),
            artist: None,
            language: None,
            lyrics: None,
            phonetic_lyrics: None,
            translations: None,
        }).unwrap();

        assert_eq!(updated.title, "New Title");
        assert_eq!(updated.artist, "Artist"); // unchanged

        // Verify persisted
        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.title, "New Title");
    }

    #[test]
    fn test_update_song_artist() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Old Artist", "en");

        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: None,
            artist: Some("New Artist".to_string()),
            language: None,
            lyrics: None,
            phonetic_lyrics: None,
            translations: None,
        }).unwrap();

        assert_eq!(updated.artist, "New Artist");
        assert_eq!(updated.title, "Song"); // unchanged
    }

    #[test]
    fn test_update_song_lyrics() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Artist", "en");

        let new_lyrics = vec!["New Line 1".to_string(), "New Line 2".to_string(), "New Line 3".to_string()];
        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: None,
            artist: None,
            language: None,
            lyrics: Some(new_lyrics.clone()),
            phonetic_lyrics: None,
            translations: None,
        }).unwrap();

        assert_eq!(updated.lyrics, new_lyrics);
    }

    #[test]
    fn test_update_song_phonetic_lyrics() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Artist", "jp");

        let phonetics = vec!["konnichiwa".to_string()];
        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: None,
            artist: None,
            language: None,
            lyrics: None,
            phonetic_lyrics: Some(phonetics.clone()),
            translations: None,
        }).unwrap();

        assert_eq!(updated.phonetic_lyrics, Some(phonetics));
    }

    #[test]
    fn test_update_song_translations() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Artist", "fr");

        let mut translations = HashMap::new();
        translations.insert("en".to_string(), vec!["Hello".to_string(), "World".to_string()]);
        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: None,
            artist: None,
            language: None,
            lyrics: None,
            phonetic_lyrics: None,
            translations: Some(translations.clone()),
        }).unwrap();

        assert_eq!(updated.translations, Some(translations));
    }

    #[test]
    fn test_update_song_multiple_fields_at_once() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Artist", "en");

        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: Some("New Title".to_string()),
            artist: Some("New Artist".to_string()),
            language: Some("fr".to_string()),
            lyrics: Some(vec!["Bonjour".to_string()]),
            phonetic_lyrics: None,
            translations: None,
        }).unwrap();

        assert_eq!(updated.title, "New Title");
        assert_eq!(updated.artist, "New Artist");
        assert_eq!(updated.language, "fr");
        assert_eq!(updated.lyrics, vec!["Bonjour".to_string()]);
    }

    #[test]
    fn test_update_song_changes_updated_at() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "Song", "Artist", "en");
        let original_updated = song.updated_at.clone();

        let updated = update_song(&conn, &song.id, UpdateSongData {
            title: Some("New".to_string()),
            artist: None,
            language: None,
            lyrics: None,
            phonetic_lyrics: None,
            translations: None,
        }).unwrap();

        // updated_at should be >= the original
        assert!(updated.updated_at >= original_updated);
    }

    #[test]
    fn test_update_song_not_found() {
        let (_tmp, conn) = setup_db();

        let result = update_song(&conn, "nonexistent-id", UpdateSongData {
            title: Some("New".to_string()),
            artist: None,
            language: None,
            lyrics: None,
            phonetic_lyrics: None,
            translations: None,
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Song not found"));
    }

    // ---- delete_song ----

    #[test]
    fn test_delete_song() {
        let (_tmp, conn) = setup_db();

        let song = insert_test_song(&conn, "To Delete", "Artist", "en");
        delete_song(&conn, &song.id).unwrap();

        let result = get_song(&conn, &song.id);
        assert!(result.is_err());
    }

    #[test]
    fn test_delete_song_not_found() {
        let (_tmp, conn) = setup_db();

        let result = delete_song(&conn, "nonexistent-id");
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Song not found"));
    }

    #[test]
    fn test_delete_song_only_removes_target() {
        let (_tmp, conn) = setup_db();

        let song1 = insert_test_song(&conn, "Keep", "Artist", "en");
        let song2 = insert_test_song(&conn, "Delete", "Artist", "en");

        delete_song(&conn, &song2.id).unwrap();

        // song1 should still exist
        let kept = get_song(&conn, &song1.id).unwrap();
        assert_eq!(kept.title, "Keep");

        // song2 should be gone
        assert!(get_song(&conn, &song2.id).is_err());
    }

    // ---- save_song / round-trip with optional JSON fields ----

    #[test]
    fn test_song_with_translations_round_trip() {
        let (_tmp, conn) = setup_db();

        let mut song = Song::new(
            "French Song".to_string(),
            "Artist".to_string(),
            "fr".to_string(),
            vec!["Bonjour le monde".to_string()],
        );

        let mut translations = HashMap::new();
        translations.insert("en".to_string(), vec!["Hello world".to_string()]);
        translations.insert("jp".to_string(), vec!["Konnichiwa sekai".to_string()]);
        song.translations = Some(translations.clone());

        save_song(&conn, &song).unwrap();

        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.translations.unwrap(), translations);
    }

    #[test]
    fn test_song_with_phonetic_round_trip() {
        let (_tmp, conn) = setup_db();

        let mut song = Song::new(
            "JP Song".to_string(),
            "Artist".to_string(),
            "jp".to_string(),
            vec!["歌".to_string()],
        );
        song.phonetic_lyrics = Some(vec!["uta".to_string()]);

        save_song(&conn, &song).unwrap();

        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.phonetic_lyrics.unwrap(), vec!["uta".to_string()]);
    }
}
