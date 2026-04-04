//! Practice session tracking service

pub use crate::models::{PracticeSession, CreateSessionData};

use crate::Result;
use rusqlite::Connection;

/// Create a new practice session
pub fn create_session(conn: &Connection, data: CreateSessionData) -> Result<PracticeSession> {
    let session = PracticeSession::new(
        data.user_id,
        data.song_id,
        data.mode,
        data.score,
        data.lines_practiced,
        data.lines_correct,
        data.duration_seconds,
    );
    
    conn.execute(
        "INSERT INTO practice_sessions 
         (id, user_id, song_id, mode, score, lines_practiced, lines_correct, 
          duration_seconds, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        rusqlite::params![
            &session.id,
            &session.user_id,
            &session.song_id,
            &session.mode,
            session.score,
            session.lines_practiced,
            session.lines_correct,
            session.duration_seconds,
            &session.created_at,
        ],
    )?;
    
    Ok(session)
}

/// Get sessions for a user, optionally limited
pub fn get_user_sessions(conn: &Connection, user_id: &str, limit: Option<i32>) -> Result<Vec<PracticeSession>> {
    let effective_limit = limit.unwrap_or(-1); // SQLite: -1 = no limit

    let mut stmt = conn.prepare(
        "SELECT id, user_id, song_id, mode, score, lines_practiced, lines_correct,
                duration_seconds, created_at
         FROM practice_sessions
         WHERE user_id = ?1
         ORDER BY created_at DESC
         LIMIT ?2"
    )?;

    let sessions = stmt.query_map(rusqlite::params![user_id, effective_limit], |row| {
        Ok(PracticeSession {
            id: row.get(0)?,
            user_id: row.get(1)?,
            song_id: row.get(2)?,
            mode: row.get(3)?,
            score: row.get(4)?,
            lines_practiced: row.get(5)?,
            lines_correct: row.get(6)?,
            duration_seconds: row.get(7)?,
            created_at: row.get(8)?,
        })
    })?.collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(sessions)
}

/// Get sessions for a specific song
pub fn get_song_sessions(conn: &Connection, song_id: &str) -> Result<Vec<PracticeSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, song_id, mode, score, lines_practiced, lines_correct,
                duration_seconds, created_at
         FROM practice_sessions
         WHERE song_id = ?1
         ORDER BY created_at DESC"
    )?;
    
    let sessions = stmt.query_map([song_id], |row| {
        Ok(PracticeSession {
            id: row.get(0)?,
            user_id: row.get(1)?,
            song_id: row.get(2)?,
            mode: row.get(3)?,
            score: row.get(4)?,
            lines_practiced: row.get(5)?,
            lines_correct: row.get(6)?,
            duration_seconds: row.get(7)?,
            created_at: row.get(8)?,
        })
    })?.collect::<rusqlite::Result<Vec<_>>>()?;
    
    Ok(sessions)
}

/// Get statistics for a user
#[derive(Debug, serde::Serialize)]
pub struct UserStats {
    pub total_sessions: i32,
    pub total_practice_time: i32,
    pub average_score: f64,
    pub total_lines_practiced: i32,
    pub total_lines_correct: i32,
}

pub fn get_user_stats(conn: &Connection, user_id: &str) -> Result<UserStats> {
    let stats = conn.query_row(
        "SELECT 
            COUNT(*) as total_sessions,
            SUM(duration_seconds) as total_time,
            AVG(score) as avg_score,
            SUM(lines_practiced) as total_practiced,
            SUM(lines_correct) as total_correct
         FROM practice_sessions
         WHERE user_id = ?1",
        [user_id],
        |row| {
            Ok(UserStats {
                total_sessions: row.get(0)?,
                total_practice_time: row.get(1).unwrap_or(0),
                average_score: row.get(2).unwrap_or(0.0),
                total_lines_practiced: row.get(3).unwrap_or(0),
                total_lines_correct: row.get(4).unwrap_or(0),
            })
        },
    )?;
    
    Ok(stats)
}

/// Get mastery level for a specific song by user
pub fn get_song_mastery(conn: &Connection, user_id: &str, song_id: &str) -> Result<f64> {
    // Calculate mastery based on recent sessions
    let mastery: Option<f64> = conn.query_row(
        "SELECT AVG(score)
         FROM (
             SELECT score FROM practice_sessions
             WHERE user_id = ?1 AND song_id = ?2
             ORDER BY created_at DESC
             LIMIT 5
         )",
        [user_id, song_id],
        |row| row.get(0),
    ).ok();
    
    Ok(mastery.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use crate::services::auth::{register, RegisterData};
    use crate::services::songs::{create_song, CreateSongData};
    use tempfile::NamedTempFile;

    #[test]
    fn test_create_and_get_session() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        // Create user
        let user = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();
        
        // Create song
        let song = create_song(&conn, CreateSongData {
            title: "Test Song".to_string(),
            artist: "Artist".to_string(),
            language: "en".to_string(),
            lyrics: vec!["Line 1".to_string()],
        }).unwrap().song;

        // Create session
        let session_data = CreateSessionData {
            user_id: user.id.clone(),
            song_id: song.id.clone(),
            mode: "karaoke".to_string(),
            score: 85.5,
            lines_practiced: 10,
            lines_correct: 8,
            duration_seconds: 120,
        };
        
        let session = create_session(&conn, session_data).unwrap();
        assert_eq!(session.score, 85.5);
        
        // Get user sessions
        let sessions = get_user_sessions(&conn, &user.id, None).unwrap();
        assert_eq!(sessions.len(), 1);
    }

    #[test]
    fn test_user_stats() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        // Create user and song
        let user = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();
        
        let song = create_song(&conn, CreateSongData {
            title: "Test Song".to_string(),
            artist: "Artist".to_string(),
            language: "en".to_string(),
            lyrics: vec!["Line 1".to_string()],
        }).unwrap().song;

        // Create multiple sessions
        for i in 1..=3 {
            let session_data = CreateSessionData {
                user_id: user.id.clone(),
                song_id: song.id.clone(),
                mode: "karaoke".to_string(),
                score: 70.0 + (i as f64 * 10.0),
                lines_practiced: 10,
                lines_correct: 7 + i,
                duration_seconds: 60,
            };
            create_session(&conn, session_data).unwrap();
        }
        
        // Get stats
        let stats = get_user_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 3);
        assert_eq!(stats.total_practice_time, 180);
        assert!(stats.average_score > 0.0);
    }
}
