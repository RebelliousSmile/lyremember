//! Practice session tracking service

pub use crate::models::{CreateSessionData, PracticeSession};

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

/// Get sessions for a user
pub fn get_user_sessions(conn: &Connection, user_id: &str) -> Result<Vec<PracticeSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, song_id, mode, score, lines_practiced, lines_correct,
                duration_seconds, created_at
         FROM practice_sessions
         WHERE user_id = ?1
         ORDER BY created_at DESC",
    )?;

    let sessions = stmt
        .query_map([user_id], |row| {
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
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

    Ok(sessions)
}

/// Get sessions for a specific song
pub fn get_song_sessions(conn: &Connection, song_id: &str) -> Result<Vec<PracticeSession>> {
    let mut stmt = conn.prepare(
        "SELECT id, user_id, song_id, mode, score, lines_practiced, lines_correct,
                duration_seconds, created_at
         FROM practice_sessions
         WHERE song_id = ?1
         ORDER BY created_at DESC",
    )?;

    let sessions = stmt
        .query_map([song_id], |row| {
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
        })?
        .collect::<rusqlite::Result<Vec<_>>>()?;

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

/// Computes the user's current daily practice streak.
///
/// A streak is the number of **consecutive days, ending today or yesterday**,
/// on which the user logged at least one practice session.
/// - 0 if there are no sessions or the most recent session is older than
///   yesterday.
/// - 1 if there's a session today.
/// - N if there's one today (or yesterday) and N-1 consecutive prior days.
///
/// Pure utility so it can be unit-tested with controlled dates.
pub fn compute_streak_from_dates(
    today: chrono::NaiveDate,
    practice_dates_desc: &[chrono::NaiveDate],
) -> i32 {
    use std::collections::BTreeSet;
    let unique: BTreeSet<_> = practice_dates_desc.iter().copied().collect();
    let mut sorted: Vec<_> = unique.into_iter().collect();
    sorted.sort_unstable_by(|a, b| b.cmp(a));

    let most_recent = match sorted.first() {
        Some(d) => *d,
        None => return 0,
    };

    let delta = (today - most_recent).num_days();
    if delta > 1 {
        return 0;
    }

    let mut streak = 1;
    let mut prev = most_recent;
    for &d in &sorted[1..] {
        if (prev - d).num_days() == 1 {
            streak += 1;
            prev = d;
        } else {
            break;
        }
    }
    streak
}

/// Returns the user's current daily practice streak, reading session dates
/// from SQLite.
pub fn get_user_streak(conn: &Connection, user_id: &str) -> Result<i32> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT DATE(created_at) AS day
         FROM practice_sessions
         WHERE user_id = ?1
         ORDER BY day DESC",
    )?;
    let dates: Vec<chrono::NaiveDate> = stmt
        .query_map([user_id], |row| {
            let s: String = row.get(0)?;
            Ok(chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").unwrap_or_default())
        })?
        .filter_map(|r| r.ok())
        .filter(|d| d != &chrono::NaiveDate::default())
        .collect();
    Ok(compute_streak_from_dates(
        chrono::Utc::now().date_naive(),
        &dates,
    ))
}

/// Returns up to `limit` song ids the user should review (lowest mastery,
/// then oldest last-practiced-date as tiebreaker). Only considers songs
/// that the user has already opened (presence of any session) — fresh
/// songs are not "recommendations" here.
pub fn get_recommendations(conn: &Connection, user_id: &str, limit: i32) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(
        "SELECT song_id, AVG(score) AS mastery, MAX(created_at) AS last
         FROM practice_sessions
         WHERE user_id = ?1
         GROUP BY song_id
         ORDER BY mastery ASC, last ASC
         LIMIT ?2",
    )?;
    let songs: Vec<String> = stmt
        .query_map(rusqlite::params![user_id, limit], |row| row.get(0))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(songs)
}

/// Get mastery level for a specific song by user
pub fn get_song_mastery(conn: &Connection, user_id: &str, song_id: &str) -> Result<f64> {
    // Calculate mastery based on recent sessions
    let mastery: Option<f64> = conn
        .query_row(
            "SELECT AVG(score)
         FROM (
             SELECT score FROM practice_sessions
             WHERE user_id = ?1 AND song_id = ?2
             ORDER BY created_at DESC
             LIMIT 5
         )",
            [user_id, song_id],
            |row| row.get(0),
        )
        .ok();

    Ok(mastery.unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use crate::models::{RegisterData, Song};
    use crate::services::auth::register;
    use crate::services::songs;
    use tempfile::NamedTempFile;

    /// Helper to create a fresh database connection for each test
    fn setup_db() -> (NamedTempFile, Connection) {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        (temp_file, conn)
    }

    /// Helper to register a test user
    fn create_test_user(conn: &Connection, username: &str) -> crate::models::User {
        register(
            conn,
            RegisterData {
                username: username.to_string(),
                email: format!("{}@example.com", username),
                password: "password123".to_string(),
            },
        )
        .unwrap()
    }

    /// Helper to insert a song directly (bypasses phonetic/translation)
    fn create_test_song(conn: &Connection, title: &str) -> Song {
        // save_song is private, so use create_song
        songs::create_song(
            conn,
            crate::models::CreateSongData {
                title: title.to_string(),
                artist: "Test Artist".to_string(),
                language: "en".to_string(),
                lyrics: vec!["Line 1".to_string(), "Line 2".to_string()],
                genius_url: None,
            },
        )
        .unwrap()
    }

    /// Helper to create a session
    fn insert_session(
        conn: &Connection,
        user_id: &str,
        song_id: &str,
        mode: &str,
        score: f64,
        lines_practiced: i32,
        lines_correct: i32,
        duration_seconds: i32,
    ) -> PracticeSession {
        create_session(
            conn,
            CreateSessionData {
                user_id: user_id.to_string(),
                song_id: song_id.to_string(),
                mode: mode.to_string(),
                score,
                lines_practiced,
                lines_correct,
                duration_seconds,
            },
        )
        .unwrap()
    }

    // ---- streak ----

    fn d(s: &str) -> chrono::NaiveDate {
        chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").unwrap()
    }

    #[test]
    fn test_streak_zero_when_no_sessions() {
        assert_eq!(compute_streak_from_dates(d("2026-05-17"), &[]), 0);
    }

    #[test]
    fn test_streak_one_when_session_today() {
        assert_eq!(
            compute_streak_from_dates(d("2026-05-17"), &[d("2026-05-17")]),
            1
        );
    }

    #[test]
    fn test_streak_three_when_three_consecutive_days_ending_today() {
        assert_eq!(
            compute_streak_from_dates(
                d("2026-05-17"),
                &[d("2026-05-17"), d("2026-05-16"), d("2026-05-15")],
            ),
            3
        );
    }

    #[test]
    fn test_streak_continues_when_last_session_was_yesterday() {
        // Yesterday-anchored streaks remain valid (user might just not have
        // practiced YET today).
        assert_eq!(
            compute_streak_from_dates(d("2026-05-17"), &[d("2026-05-16"), d("2026-05-15")],),
            2
        );
    }

    #[test]
    fn test_streak_resets_when_gap_of_more_than_one_day() {
        assert_eq!(
            compute_streak_from_dates(d("2026-05-17"), &[d("2026-05-10")]),
            0
        );
    }

    #[test]
    fn test_streak_breaks_at_first_gap() {
        // Today, yesterday, then a 2-day gap before another batch — only the
        // first 2 days count.
        assert_eq!(
            compute_streak_from_dates(
                d("2026-05-17"),
                &[
                    d("2026-05-17"),
                    d("2026-05-16"),
                    d("2026-05-13"),
                    d("2026-05-12"),
                ],
            ),
            2
        );
    }

    #[test]
    fn test_streak_deduplicates_multiple_sessions_per_day() {
        // 4 entries on 2 unique days should yield streak 2.
        assert_eq!(
            compute_streak_from_dates(
                d("2026-05-17"),
                &[
                    d("2026-05-17"),
                    d("2026-05-17"),
                    d("2026-05-16"),
                    d("2026-05-16"),
                ],
            ),
            2
        );
    }

    // ---- recommendations ----

    #[test]
    fn test_recommendations_orders_by_lowest_mastery() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "u");
        let s_high = create_test_song(&conn, "High mastery");
        let s_low = create_test_song(&conn, "Low mastery");
        let s_mid = create_test_song(&conn, "Mid mastery");
        insert_session(&conn, &user.id, &s_high.id, "mcq", 95.0, 10, 9, 60);
        insert_session(&conn, &user.id, &s_low.id, "mcq", 20.0, 10, 2, 60);
        insert_session(&conn, &user.id, &s_mid.id, "mcq", 60.0, 10, 6, 60);

        let recs = get_recommendations(&conn, &user.id, 3).unwrap();
        assert_eq!(recs, vec![s_low.id, s_mid.id, s_high.id]);
    }

    #[test]
    fn test_recommendations_respects_limit() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "u");
        for i in 0..5 {
            let s = create_test_song(&conn, &format!("Song {}", i));
            insert_session(&conn, &user.id, &s.id, "mcq", 50.0, 10, 5, 60);
        }
        let recs = get_recommendations(&conn, &user.id, 2).unwrap();
        assert_eq!(recs.len(), 2);
    }

    #[test]
    fn test_create_session_returns_correct_data() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Test Song");

        let session = insert_session(&conn, &user.id, &song.id, "karaoke", 85.5, 10, 8, 120);

        assert_eq!(session.user_id, user.id);
        assert_eq!(session.song_id, song.id);
        assert_eq!(session.mode, "karaoke");
        assert_eq!(session.score, 85.5);
        assert_eq!(session.lines_practiced, 10);
        assert_eq!(session.lines_correct, 8);
        assert_eq!(session.duration_seconds, 120);
        assert!(!session.id.is_empty());
        assert!(!session.created_at.is_empty());
    }

    #[test]
    fn test_create_session_generates_unique_ids() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Test Song");

        let s1 = insert_session(&conn, &user.id, &song.id, "karaoke", 80.0, 10, 8, 60);
        let s2 = insert_session(&conn, &user.id, &song.id, "karaoke", 90.0, 10, 9, 60);

        assert_ne!(s1.id, s2.id);
    }

    #[test]
    fn test_create_session_different_modes() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Test Song");

        for mode in &["karaoke", "fill-blank", "mcq", "oral"] {
            let session = insert_session(&conn, &user.id, &song.id, mode, 75.0, 5, 4, 30);
            assert_eq!(session.mode, *mode);
        }

        let sessions = get_user_sessions(&conn, &user.id).unwrap();
        assert_eq!(sessions.len(), 4);
    }

    #[test]
    fn test_create_session_zero_score() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Test Song");

        let session = insert_session(&conn, &user.id, &song.id, "mcq", 0.0, 5, 0, 30);
        assert_eq!(session.score, 0.0);
        assert_eq!(session.lines_correct, 0);
    }

    #[test]
    fn test_create_session_perfect_score() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Test Song");

        let session = insert_session(&conn, &user.id, &song.id, "karaoke", 100.0, 10, 10, 60);
        assert_eq!(session.score, 100.0);
    }

    // ---- get_user_sessions ----

    #[test]
    fn test_get_user_sessions_empty() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");

        let sessions = get_user_sessions(&conn, &user.id).unwrap();
        assert!(sessions.is_empty());
    }

    #[test]
    fn test_get_user_sessions_returns_correct_user_sessions() {
        let (_tmp, conn) = setup_db();
        let user1 = create_test_user(&conn, "alice");
        let user2 = create_test_user(&conn, "bob");
        let song = create_test_song(&conn, "Song");

        insert_session(&conn, &user1.id, &song.id, "karaoke", 80.0, 10, 8, 60);
        insert_session(&conn, &user1.id, &song.id, "mcq", 90.0, 10, 9, 60);
        insert_session(&conn, &user2.id, &song.id, "karaoke", 70.0, 10, 7, 60);

        let alice_sessions = get_user_sessions(&conn, &user1.id).unwrap();
        assert_eq!(alice_sessions.len(), 2);
        assert!(alice_sessions.iter().all(|s| s.user_id == user1.id));

        let bob_sessions = get_user_sessions(&conn, &user2.id).unwrap();
        assert_eq!(bob_sessions.len(), 1);
        assert_eq!(bob_sessions[0].user_id, user2.id);
    }

    #[test]
    fn test_get_user_sessions_ordered_by_created_at_desc() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        let s1 = insert_session(&conn, &user.id, &song.id, "karaoke", 70.0, 10, 7, 60);
        let _s2 = insert_session(&conn, &user.id, &song.id, "karaoke", 80.0, 10, 8, 60);
        let s3 = insert_session(&conn, &user.id, &song.id, "karaoke", 90.0, 10, 9, 60);

        let sessions = get_user_sessions(&conn, &user.id).unwrap();
        assert_eq!(sessions.len(), 3);
        // Most recent first
        assert_eq!(sessions[0].id, s3.id);
        assert_eq!(sessions[2].id, s1.id);
    }

    // ---- get_song_sessions ----

    #[test]
    fn test_get_song_sessions_empty() {
        let (_tmp, conn) = setup_db();
        let song = create_test_song(&conn, "Song");

        let sessions = get_song_sessions(&conn, &song.id).unwrap();
        assert!(sessions.is_empty());
    }

    #[test]
    fn test_get_song_sessions_returns_sessions_for_song() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song1 = create_test_song(&conn, "Song 1");
        let song2 = create_test_song(&conn, "Song 2");

        insert_session(&conn, &user.id, &song1.id, "karaoke", 80.0, 10, 8, 60);
        insert_session(&conn, &user.id, &song1.id, "mcq", 90.0, 10, 9, 60);
        insert_session(&conn, &user.id, &song2.id, "karaoke", 70.0, 10, 7, 60);

        let song1_sessions = get_song_sessions(&conn, &song1.id).unwrap();
        assert_eq!(song1_sessions.len(), 2);
        assert!(song1_sessions.iter().all(|s| s.song_id == song1.id));

        let song2_sessions = get_song_sessions(&conn, &song2.id).unwrap();
        assert_eq!(song2_sessions.len(), 1);
    }

    // ---- get_user_stats ----

    #[test]
    fn test_get_user_stats_zero_sessions() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");

        let stats = get_user_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 0);
        assert_eq!(stats.total_practice_time, 0);
        assert_eq!(stats.average_score, 0.0);
        assert_eq!(stats.total_lines_practiced, 0);
        assert_eq!(stats.total_lines_correct, 0);
    }

    #[test]
    fn test_get_user_stats_single_session() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        insert_session(&conn, &user.id, &song.id, "karaoke", 85.0, 10, 8, 120);

        let stats = get_user_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 1);
        assert_eq!(stats.total_practice_time, 120);
        assert_eq!(stats.average_score, 85.0);
        assert_eq!(stats.total_lines_practiced, 10);
        assert_eq!(stats.total_lines_correct, 8);
    }

    #[test]
    fn test_get_user_stats_multiple_sessions() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        // scores: 80, 90, 100 -> avg = 90
        // duration: 60 * 3 = 180
        // lines_practiced: 10 * 3 = 30
        // lines_correct: 8 + 9 + 10 = 27
        insert_session(&conn, &user.id, &song.id, "karaoke", 80.0, 10, 8, 60);
        insert_session(&conn, &user.id, &song.id, "karaoke", 90.0, 10, 9, 60);
        insert_session(&conn, &user.id, &song.id, "karaoke", 100.0, 10, 10, 60);

        let stats = get_user_stats(&conn, &user.id).unwrap();
        assert_eq!(stats.total_sessions, 3);
        assert_eq!(stats.total_practice_time, 180);
        assert!((stats.average_score - 90.0).abs() < 0.01);
        assert_eq!(stats.total_lines_practiced, 30);
        assert_eq!(stats.total_lines_correct, 27);
    }

    #[test]
    fn test_get_user_stats_does_not_include_other_users() {
        let (_tmp, conn) = setup_db();
        let user1 = create_test_user(&conn, "alice");
        let user2 = create_test_user(&conn, "bob");
        let song = create_test_song(&conn, "Song");

        insert_session(&conn, &user1.id, &song.id, "karaoke", 80.0, 10, 8, 60);
        insert_session(&conn, &user2.id, &song.id, "karaoke", 100.0, 10, 10, 120);

        let stats1 = get_user_stats(&conn, &user1.id).unwrap();
        assert_eq!(stats1.total_sessions, 1);
        assert_eq!(stats1.total_practice_time, 60);
        assert_eq!(stats1.average_score, 80.0);

        let stats2 = get_user_stats(&conn, &user2.id).unwrap();
        assert_eq!(stats2.total_sessions, 1);
        assert_eq!(stats2.total_practice_time, 120);
        assert_eq!(stats2.average_score, 100.0);
    }

    // ---- get_song_mastery ----

    #[test]
    fn test_get_song_mastery_no_sessions() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        let mastery = get_song_mastery(&conn, &user.id, &song.id).unwrap();
        assert_eq!(mastery, 0.0);
    }

    #[test]
    fn test_get_song_mastery_single_session() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        insert_session(&conn, &user.id, &song.id, "karaoke", 75.0, 10, 7, 60);

        let mastery = get_song_mastery(&conn, &user.id, &song.id).unwrap();
        assert!((mastery - 75.0).abs() < 0.01);
    }

    #[test]
    fn test_get_song_mastery_averages_last_5_sessions() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song = create_test_song(&conn, "Song");

        // Insert 7 sessions: 50, 60, 70, 80, 90, 95, 100
        // Last 5 should be 70, 80, 90, 95, 100 -> avg = 87
        for score in &[50.0, 60.0, 70.0, 80.0, 90.0, 95.0, 100.0] {
            insert_session(&conn, &user.id, &song.id, "karaoke", *score, 10, 8, 60);
        }

        let mastery = get_song_mastery(&conn, &user.id, &song.id).unwrap();
        // Average of last 5: (70+80+90+95+100)/5 = 87.0
        assert!((mastery - 87.0).abs() < 0.01);
    }

    #[test]
    fn test_get_song_mastery_different_songs_independent() {
        let (_tmp, conn) = setup_db();
        let user = create_test_user(&conn, "testuser");
        let song1 = create_test_song(&conn, "Song 1");
        let song2 = create_test_song(&conn, "Song 2");

        insert_session(&conn, &user.id, &song1.id, "karaoke", 90.0, 10, 9, 60);
        insert_session(&conn, &user.id, &song2.id, "karaoke", 50.0, 10, 5, 60);

        let mastery1 = get_song_mastery(&conn, &user.id, &song1.id).unwrap();
        let mastery2 = get_song_mastery(&conn, &user.id, &song2.id).unwrap();

        assert!((mastery1 - 90.0).abs() < 0.01);
        assert!((mastery2 - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_get_song_mastery_different_users_independent() {
        let (_tmp, conn) = setup_db();
        let user1 = create_test_user(&conn, "alice");
        let user2 = create_test_user(&conn, "bob");
        let song = create_test_song(&conn, "Song");

        insert_session(&conn, &user1.id, &song.id, "karaoke", 90.0, 10, 9, 60);
        insert_session(&conn, &user2.id, &song.id, "karaoke", 40.0, 10, 4, 60);

        let mastery1 = get_song_mastery(&conn, &user1.id, &song.id).unwrap();
        let mastery2 = get_song_mastery(&conn, &user2.id, &song.id).unwrap();

        assert!((mastery1 - 90.0).abs() < 0.01);
        assert!((mastery2 - 40.0).abs() < 0.01);
    }
}
