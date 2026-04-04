//! SQLite database implementation

use crate::Result;
use rusqlite::Connection;
use std::path::Path;

/// Current schema version
const SCHEMA_VERSION: i32 = 1;

/// Initialize the database with schema and run migrations
pub fn init_database<P: AsRef<Path>>(path: P) -> Result<Connection> {
    let conn = Connection::open(path)?;

    // Create schema_version table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS schema_version (
            version INTEGER NOT NULL
        )",
        [],
    )?;

    // Get current version (0 if fresh database)
    let current_version: i32 = conn
        .query_row("SELECT COALESCE(MAX(version), 0) FROM schema_version", [], |row| row.get(0))
        .unwrap_or(0);

    // Create base tables (idempotent with IF NOT EXISTS)
    create_tables(&conn)?;

    // Run migrations
    run_migrations(&conn, current_version)?;

    Ok(conn)
}

/// Run all migrations from current_version to SCHEMA_VERSION
fn run_migrations(conn: &Connection, current_version: i32) -> Result<()> {
    if current_version >= SCHEMA_VERSION {
        return Ok(());
    }

    // Migration 0 → 1: initial schema (tables already created by create_tables)
    if current_version < 1 {
        // Base schema is handled by create_tables(). Future migrations go here:
        // if current_version < 2 { migrate_to_v2(conn)?; }
        // if current_version < 3 { migrate_to_v3(conn)?; }
    }

    // Record the new version
    conn.execute("DELETE FROM schema_version", [])?;
    conn.execute("INSERT INTO schema_version (version) VALUES (?1)", [SCHEMA_VERSION])?;

    Ok(())
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

    #[test]
    fn test_init_database() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        // Verify tables were created
        let table_count: i32 = conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table'",
                [],
                |row| row.get(0),
            )
            .unwrap();
        
        assert!(table_count >= 4); // At least 4 tables
    }
}
