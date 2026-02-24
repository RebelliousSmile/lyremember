//! Practice session model

use serde::{Deserialize, Serialize};

/// Practice session tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSession {
    pub id: String,
    pub user_id: String,
    pub song_id: String,
    pub mode: String, // 'karaoke', 'fill-blank', 'mcq', 'oral'
    pub score: f64, // 0.0 to 100.0
    pub lines_practiced: i32,
    pub lines_correct: i32,
    pub duration_seconds: i32,
    pub created_at: String,
}

impl PracticeSession {
    /// Create a new practice session
    pub fn new(
        user_id: String,
        song_id: String,
        mode: String,
        score: f64,
        lines_practiced: i32,
        lines_correct: i32,
        duration_seconds: i32,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            song_id,
            mode,
            score,
            lines_practiced,
            lines_correct,
            duration_seconds,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// Data for creating a practice session
#[derive(Debug, Deserialize)]
pub struct CreateSessionData {
    pub user_id: String,
    pub song_id: String,
    pub mode: String,
    pub score: f64,
    pub lines_practiced: i32,
    pub lines_correct: i32,
    pub duration_seconds: i32,
}
