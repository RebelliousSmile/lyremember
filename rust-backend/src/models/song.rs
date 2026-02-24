//! Song model

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Song with lyrics in multiple forms
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub language: String, // 'fr', 'en', 'jp', 'kr'
    pub lyrics: Vec<String>, // Original lyrics (line by line)
    
    // Generated and cached data
    pub phonetic_lyrics: Option<Vec<String>>, // Romanized/IPA (cached)
    pub translations: Option<HashMap<String, Vec<String>>>, // e.g. {"en": [...], "fr": [...]}
    
    // Metadata
    pub genius_id: Option<String>,
    pub genius_url: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Song {
    /// Create a new song with generated UUID
    pub fn new(
        title: String,
        artist: String,
        language: String,
        lyrics: Vec<String>,
    ) -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title,
            artist,
            language,
            lyrics,
            phonetic_lyrics: None,
            translations: None,
            genius_id: None,
            genius_url: None,
            created_at: now.clone(),
            updated_at: now,
        }
    }
}

/// Data for creating a new song
#[derive(Debug, Deserialize)]
pub struct CreateSongData {
    pub title: String,
    pub artist: String,
    pub language: String,
    pub lyrics: Vec<String>,
}

/// Data for updating a song
#[derive(Debug, Deserialize)]
pub struct UpdateSongData {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub language: Option<String>,
    pub lyrics: Option<Vec<String>>,
    pub phonetic_lyrics: Option<Vec<String>>,
    pub translations: Option<HashMap<String, Vec<String>>>,
}
