//! SQLite database implementation

use crate::Result;
use rusqlite::Connection;
use std::path::Path;

/// Initialize the database with schema
pub fn init_database<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;
    
    // Create tables
    create_tables(&conn)?;
    
    Ok(conn)
}

/// Get a database connection (helper for opening existing database)
pub fn get_connection<P: AsRef<Path>>(path: P) -> Result<Connection> {
    Ok(Connection::open(path)?)
}

/// Create all database tables
fn create_tables(conn: &Connection) -> Result<()> {
    // Users table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            email TEXT NOT NULL,
            password_hash TEXT NOT NULL,
            genius_token TEXT,
            created_at TEXT NOT NULL
        )",
        [],
    )?;

    // Songs table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS songs (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            artist TEXT NOT NULL,
            language TEXT NOT NULL,
            lyrics TEXT NOT NULL,
            phonetic_lyrics TEXT,
            translations TEXT,
            genius_id TEXT,
            genius_url TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
        [],
    )?;

    // User-Songs junction table (user's repertoire)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS user_songs (
            user_id TEXT NOT NULL,
            song_id TEXT NOT NULL,
            added_at TEXT NOT NULL,
            PRIMARY KEY (user_id, song_id),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Practice sessions table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS practice_sessions (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            song_id TEXT NOT NULL,
            mode TEXT NOT NULL,
            score REAL NOT NULL,
            lines_practiced INTEGER NOT NULL,
            lines_correct INTEGER NOT NULL,
            duration_seconds INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (song_id) REFERENCES songs(id) ON DELETE CASCADE
        )",
        [],
    )?;

    // Create indexes for performance
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_users_username ON users(username)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_songs_language ON songs(language)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_user_songs_user ON user_songs(user_id)",
        [],
    )?;
    
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_sessions_user ON practice_sessions(user_id)",
        [],
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn setup_db() -> (NamedTempFile, Connection) {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        (temp_file, conn)
    }

    /// Helper: get all table names from the database
    fn get_table_names(conn: &Connection) -> Vec<String> {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name"
        ).unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<String>>>()
            .unwrap()
    }

    /// Helper: get all index names from the database
    fn get_index_names(conn: &Connection) -> Vec<String> {
        let mut stmt = conn.prepare(
            "SELECT name FROM sqlite_master WHERE type='index' AND name NOT LIKE 'sqlite_%' ORDER BY name"
        ).unwrap();
        stmt.query_map([], |row| row.get(0))
            .unwrap()
            .collect::<rusqlite::Result<Vec<String>>>()
            .unwrap()
    }

    #[test]
    fn test_init_database_creates_all_tables() {
        let (_tmp, conn) = setup_db();
        let tables = get_table_names(&conn);

        assert!(tables.contains(&"users".to_string()), "missing users table");
        assert!(tables.contains(&"songs".to_string()), "missing songs table");
        assert!(tables.contains(&"user_songs".to_string()), "missing user_songs table");
        assert!(tables.contains(&"practice_sessions".to_string()), "missing practice_sessions table");
    }

    #[test]
    fn test_init_database_creates_indexes() {
        let (_tmp, conn) = setup_db();
        let indexes = get_index_names(&conn);

        assert!(indexes.contains(&"idx_users_username".to_string()), "missing idx_users_username");
        assert!(indexes.contains(&"idx_songs_language".to_string()), "missing idx_songs_language");
        assert!(indexes.contains(&"idx_user_songs_user".to_string()), "missing idx_user_songs_user");
        assert!(indexes.contains(&"idx_sessions_user".to_string()), "missing idx_sessions_user");
    }

    #[test]
    fn test_init_database_is_idempotent() {
        let temp_file = NamedTempFile::new().unwrap();

        // Initialize twice on same path
        let conn = init_database(temp_file.path()).unwrap();
        drop(conn);
        let conn2 = init_database(temp_file.path()).unwrap();

        let tables = get_table_names(&conn2);
        assert_eq!(tables.len(), 4);
    }

    #[test]
    fn test_get_connection_opens_existing_db() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();

        // Insert a user to verify data persists
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u1', 'alice', 'alice@example.com', 'hash', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();
        drop(conn);

        // Re-open with get_connection
        let conn2 = get_connection(temp_file.path()).unwrap();
        let username: String = conn2.query_row(
            "SELECT username FROM users WHERE id = 'u1'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(username, "alice");
    }

    #[test]
    fn test_users_table_schema() {
        let (_tmp, conn) = setup_db();

        // Verify we can insert and read all columns of the users table
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, genius_token, created_at)
             VALUES ('u1', 'alice', 'alice@example.com', 'hash123', 'token_abc', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        let (id, username, email, pw_hash, genius, created): (String, String, String, String, Option<String>, String) =
            conn.query_row(
                "SELECT id, username, email, password_hash, genius_token, created_at FROM users WHERE id = 'u1'",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?)),
            ).unwrap();

        assert_eq!(id, "u1");
        assert_eq!(username, "alice");
        assert_eq!(email, "alice@example.com");
        assert_eq!(pw_hash, "hash123");
        assert_eq!(genius, Some("token_abc".to_string()));
        assert_eq!(created, "2024-01-01T00:00:00Z");
    }

    #[test]
    fn test_users_table_username_unique_constraint() {
        let (_tmp, conn) = setup_db();

        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u1', 'alice', 'alice@example.com', 'hash', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        let result = conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u2', 'alice', 'other@example.com', 'hash', '2024-01-01T00:00:00Z')",
            [],
        );
        assert!(result.is_err(), "Duplicate username should fail");
    }

    #[test]
    fn test_songs_table_schema() {
        let (_tmp, conn) = setup_db();

        conn.execute(
            "INSERT INTO songs (id, title, artist, language, lyrics, phonetic_lyrics, translations, genius_id, genius_url, created_at, updated_at)
             VALUES ('s1', 'Song', 'Artist', 'en', '[\"line1\"]', NULL, NULL, NULL, NULL, '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        let title: String = conn.query_row(
            "SELECT title FROM songs WHERE id = 's1'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert_eq!(title, "Song");
    }

    #[test]
    fn test_user_songs_composite_primary_key() {
        let (_tmp, conn) = setup_db();

        // Create prerequisite rows
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u1', 'alice', 'a@e.com', 'h', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO songs (id, title, artist, language, lyrics, created_at, updated_at)
             VALUES ('s1', 'Song', 'Art', 'en', '[]', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO user_songs (user_id, song_id, added_at) VALUES ('u1', 's1', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        // Duplicate should fail (composite PK)
        let result = conn.execute(
            "INSERT INTO user_songs (user_id, song_id, added_at) VALUES ('u1', 's1', '2024-01-02T00:00:00Z')",
            [],
        );
        assert!(result.is_err(), "Duplicate user_songs entry should fail");
    }

    #[test]
    fn test_practice_sessions_table_schema() {
        let (_tmp, conn) = setup_db();

        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u1', 'alice', 'a@e.com', 'h', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO songs (id, title, artist, language, lyrics, created_at, updated_at)
             VALUES ('s1', 'Song', 'Art', 'en', '[]', '2024-01-01T00:00:00Z', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        conn.execute(
            "INSERT INTO practice_sessions (id, user_id, song_id, mode, score, lines_practiced, lines_correct, duration_seconds, created_at)
             VALUES ('p1', 'u1', 's1', 'karaoke', 85.5, 10, 8, 120, '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        let (mode, score, lines_p, lines_c, dur): (String, f64, i32, i32, i32) = conn.query_row(
            "SELECT mode, score, lines_practiced, lines_correct, duration_seconds FROM practice_sessions WHERE id = 'p1'",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        ).unwrap();

        assert_eq!(mode, "karaoke");
        assert!((score - 85.5).abs() < 0.01);
        assert_eq!(lines_p, 10);
        assert_eq!(lines_c, 8);
        assert_eq!(dur, 120);
    }

    #[test]
    fn test_genius_token_nullable() {
        let (_tmp, conn) = setup_db();

        // genius_token should be nullable
        conn.execute(
            "INSERT INTO users (id, username, email, password_hash, created_at)
             VALUES ('u1', 'alice', 'a@e.com', 'h', '2024-01-01T00:00:00Z')",
            [],
        ).unwrap();

        let token: Option<String> = conn.query_row(
            "SELECT genius_token FROM users WHERE id = 'u1'",
            [],
            |row| row.get(0),
        ).unwrap();
        assert!(token.is_none());
    }
}
