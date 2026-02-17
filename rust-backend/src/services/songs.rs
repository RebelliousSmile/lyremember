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
        match phonetic::generate_phonetic(data.lyrics.clone(), &song.language) {
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
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_and_get_song() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        let data = CreateSongData {
            title: "Test Song".to_string(),
            artist: "Test Artist".to_string(),
            language: "en".to_string(),
            lyrics: vec!["Line 1".to_string(), "Line 2".to_string()],
        };
        
        let song = create_song(&conn, data).unwrap();
        assert_eq!(song.title, "Test Song");
        
        let retrieved = get_song(&conn, &song.id).unwrap();
        assert_eq!(retrieved.title, song.title);
        assert_eq!(retrieved.lyrics.len(), 2);
    }

    #[test]
    fn test_get_all_songs() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        // Create multiple songs
        for i in 1..=3 {
            let data = CreateSongData {
                title: format!("Song {}", i),
                artist: "Artist".to_string(),
                language: "en".to_string(),
                lyrics: vec![format!("Line {}", i)],
            };
            create_song(&conn, data).unwrap();
        }
        
        let songs = get_all_songs(&conn).unwrap();
        assert_eq!(songs.len(), 3);
    }

    #[test]
    fn test_delete_song() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        let data = CreateSongData {
            title: "To Delete".to_string(),
            artist: "Artist".to_string(),
            language: "en".to_string(),
            lyrics: vec!["Line".to_string()],
        };
        
        let song = create_song(&conn, data).unwrap();
        delete_song(&conn, &song.id).unwrap();
        
        let result = get_song(&conn, &song.id);
        assert!(result.is_err());
    }
}
