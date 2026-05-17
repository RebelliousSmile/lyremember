//! Authentication service

pub use crate::models::{LoginCredentials, RegisterData, User};

use crate::{Error, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

const JWT_SECRET_ENV: &str = "LYREMEMBER_JWT_SECRET";

/// Resolves the JWT signing secret from a provided env value (typically
/// `std::env::var(JWT_SECRET_ENV).ok()`). If the value is `None` or empty,
/// returns a 32-byte random ephemeral secret and logs a warning — sessions
/// will not survive a process restart in that case.
///
/// Split from `jwt_secret()` so it can be unit-tested without mutating the
/// process environment.
fn jwt_secret_from_env(env_value: Option<String>) -> Vec<u8> {
    match env_value {
        Some(s) if !s.is_empty() => s.into_bytes(),
        _ => {
            eprintln!(
                "[WARN] {} is not set — generating an ephemeral JWT secret. \
                 JWT tokens will become invalid on process restart. \
                 Set {} in production.",
                JWT_SECRET_ENV, JWT_SECRET_ENV
            );
            let mut s = Vec::with_capacity(32);
            s.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
            s.extend_from_slice(uuid::Uuid::new_v4().as_bytes());
            s
        }
    }
}

/// Returns the JWT signing secret, resolved once per process from the
/// `LYREMEMBER_JWT_SECRET` env var with an ephemeral fallback.
fn jwt_secret() -> &'static [u8] {
    static SECRET: OnceLock<Vec<u8>> = OnceLock::new();
    SECRET
        .get_or_init(|| jwt_secret_from_env(std::env::var(JWT_SECRET_ENV).ok()))
        .as_slice()
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    username: String,
    exp: usize, // Expiration time
}

/// Register a new user
pub fn register(conn: &Connection, data: RegisterData) -> Result<User> {
    // Validate input
    if data.username.trim().is_empty() {
        return Err(Error::Auth("Username cannot be empty".to_string()));
    }
    if data.email.trim().is_empty() {
        return Err(Error::Auth("Email cannot be empty".to_string()));
    }
    if data.password.len() < 8 {
        return Err(Error::Auth(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if username already exists
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM users WHERE username = ?1)",
            [&data.username],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if exists {
        return Err(Error::Auth("Username already taken".to_string()));
    }

    // Hash password
    let password_hash = hash(&data.password, DEFAULT_COST)
        .map_err(|e| Error::Auth(format!("Failed to hash password: {}", e)))?;

    // Create user
    let user = User::new(data.username, data.email, password_hash);

    // Insert into database
    conn.execute(
        "INSERT INTO users (id, username, email, password_hash, genius_token, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        rusqlite::params![
            &user.id,
            &user.username,
            &user.email,
            &user.password_hash,
            &user.genius_token,
            &user.created_at,
        ],
    )?;

    Ok(user)
}

/// Login user and return JWT token
pub fn login(conn: &Connection, credentials: LoginCredentials) -> Result<(User, String)> {
    // Get user from database
    let mut stmt = conn.prepare(
        "SELECT id, username, email, password_hash, genius_token, created_at
         FROM users WHERE username = ?1",
    )?;

    let user = stmt
        .query_row([&credentials.username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                genius_token: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|_| Error::Auth("Invalid username or password".to_string()))?;

    // Verify password
    let valid = verify(&credentials.password, &user.password_hash)
        .map_err(|e| Error::Auth(format!("Password verification failed: {}", e)))?;

    if !valid {
        return Err(Error::Auth("Invalid username or password".to_string()));
    }

    // Generate JWT token
    let token = generate_token(&user)?;

    Ok((user, token))
}

/// Generate JWT token for user
fn generate_token(user: &User) -> Result<String> {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::days(30))
        .ok_or_else(|| Error::Auth("Failed to calculate expiration".to_string()))?
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.clone(),
        username: user.username.clone(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret()),
    )
    .map_err(|e| Error::Auth(format!("Failed to generate token: {}", e)))?;

    Ok(token)
}

/// Verify JWT token and return user ID
pub fn verify_token(token: &str) -> Result<String> {
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(token, &DecodingKey::from_secret(jwt_secret()), &validation)
        .map_err(|e| Error::Auth(format!("Invalid token: {}", e)))?;

    Ok(token_data.claims.sub)
}

/// Login as guest: create or retrieve a local guest account
pub fn login_as_guest(conn: &Connection) -> Result<(User, String)> {
    let guest_username = "guest";

    // Check if guest user already exists
    let existing: Option<User> = conn
        .prepare(
            "SELECT id, username, email, password_hash, genius_token, created_at
         FROM users WHERE username = ?1",
        )?
        .query_row([guest_username], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                genius_token: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .ok();

    let user = match existing {
        Some(u) => u,
        None => {
            let password_hash = hash("guest-local-account", DEFAULT_COST)
                .map_err(|e| Error::Auth(format!("Failed to hash password: {}", e)))?;
            let user = User::new(
                guest_username.to_string(),
                "guest@local".to_string(),
                password_hash,
            );
            conn.execute(
                "INSERT INTO users (id, username, email, password_hash, genius_token, created_at)
                 VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    &user.id,
                    &user.username,
                    &user.email,
                    &user.password_hash,
                    &user.genius_token,
                    &user.created_at
                ],
            )?;
            user
        }
    };

    let token = generate_token(&user)?;
    Ok((user, token))
}

/// Get user by ID
pub fn get_user_by_id(conn: &Connection, user_id: &str) -> Result<User> {
    let mut stmt = conn.prepare(
        "SELECT id, username, email, password_hash, genius_token, created_at
         FROM users WHERE id = ?1",
    )?;

    let user = stmt
        .query_row([user_id], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
                email: row.get(2)?,
                password_hash: row.get(3)?,
                genius_token: row.get(4)?,
                created_at: row.get(5)?,
            })
        })
        .map_err(|_| Error::NotFound("User not found".to_string()))?;

    Ok(user)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::init_database;
    use tempfile::NamedTempFile;

    /// Helper to create a fresh database connection for each test
    fn setup_db() -> (tempfile::NamedTempFile, Connection) {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        (temp_file, conn)
    }

    /// Helper to register a default test user
    fn register_test_user(conn: &Connection) -> User {
        register(
            conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .unwrap()
    }

    #[test]
    fn test_register_creates_user_with_correct_fields() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        assert_eq!(user.username, "testuser");
        assert_eq!(user.email, "test@example.com");
        assert!(!user.id.is_empty());
        assert!(!user.password_hash.is_empty());
        assert!(user.genius_token.is_none());
        assert!(!user.created_at.is_empty());
    }

    #[test]
    fn test_register_hashes_password() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        // The stored hash should not equal the plaintext password
        assert_ne!(user.password_hash, "password123");
        // bcrypt hashes start with $2b$
        assert!(user.password_hash.starts_with("$2b$"));
    }

    #[test]
    fn test_register_and_login_roundtrip() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        let credentials = LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };

        let (logged_user, token) = login(&conn, credentials).unwrap();
        assert_eq!(logged_user.id, user.id);
        assert_eq!(logged_user.username, "testuser");
        assert_eq!(logged_user.email, "test@example.com");
        assert!(!token.is_empty());
    }

    #[test]
    fn test_verify_token_returns_correct_user_id() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        let credentials = LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        let (_logged_user, token) = login(&conn, credentials).unwrap();

        let user_id = verify_token(&token).unwrap();
        assert_eq!(user_id, user.id);
    }

    #[test]
    fn test_verify_token_invalid_token() {
        let result = verify_token("this.is.not.a.valid.token");
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Invalid token") || err_msg.contains("Authentication error"));
    }

    #[test]
    fn test_verify_token_empty_string() {
        let result = verify_token("");
        assert!(result.is_err());
    }

    #[test]
    fn test_register_duplicate_username() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);

        let result = register(
            &conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "other@example.com".to_string(),
                password: "password456".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username already taken"));
    }

    #[test]
    fn test_register_same_email_different_username_ok() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);

        // Same email but different username should succeed (no unique constraint on email)
        let result = register(
            &conn,
            RegisterData {
                username: "anotheruser".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_empty_username() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username cannot be empty"));
    }

    #[test]
    fn test_register_whitespace_only_username() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "   ".to_string(),
                email: "test@example.com".to_string(),
                password: "password123".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username cannot be empty"));
    }

    #[test]
    fn test_register_empty_email() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "".to_string(),
                password: "password123".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Email cannot be empty"));
    }

    #[test]
    fn test_register_short_password() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password: "short".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Password must be at least 8 characters"));
    }

    #[test]
    fn test_register_password_exactly_8_chars() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password: "12345678".to_string(),
            },
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_password_7_chars_fails() {
        let (_tmp, conn) = setup_db();
        let result = register(
            &conn,
            RegisterData {
                username: "testuser".to_string(),
                email: "test@example.com".to_string(),
                password: "1234567".to_string(),
            },
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_login_wrong_password() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);

        let result = login(
            &conn,
            LoginCredentials {
                username: "testuser".to_string(),
                password: "wrongpassword".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Invalid username or password"));
    }

    #[test]
    fn test_login_nonexistent_user() {
        let (_tmp, conn) = setup_db();

        let result = login(
            &conn,
            LoginCredentials {
                username: "noone".to_string(),
                password: "password123".to_string(),
            },
        );
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Invalid username or password"));
    }

    #[test]
    fn test_get_user_by_id_success() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        let found = get_user_by_id(&conn, &user.id).unwrap();
        assert_eq!(found.id, user.id);
        assert_eq!(found.username, "testuser");
        assert_eq!(found.email, "test@example.com");
    }

    #[test]
    fn test_get_user_by_id_not_found() {
        let (_tmp, conn) = setup_db();

        let result = get_user_by_id(&conn, "nonexistent-uuid");
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("User not found"));
    }

    #[test]
    fn test_register_multiple_users() {
        let (_tmp, conn) = setup_db();

        let user1 = register(
            &conn,
            RegisterData {
                username: "alice".to_string(),
                email: "alice@example.com".to_string(),
                password: "password123".to_string(),
            },
        )
        .unwrap();

        let user2 = register(
            &conn,
            RegisterData {
                username: "bob".to_string(),
                email: "bob@example.com".to_string(),
                password: "password456".to_string(),
            },
        )
        .unwrap();

        assert_ne!(user1.id, user2.id);

        // Both can log in independently
        let (u1, _t1) = login(
            &conn,
            LoginCredentials {
                username: "alice".to_string(),
                password: "password123".to_string(),
            },
        )
        .unwrap();
        assert_eq!(u1.id, user1.id);

        let (u2, _t2) = login(
            &conn,
            LoginCredentials {
                username: "bob".to_string(),
                password: "password456".to_string(),
            },
        )
        .unwrap();
        assert_eq!(u2.id, user2.id);
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        let (_, token) = login(
            &conn,
            LoginCredentials {
                username: "testuser".to_string(),
                password: "password123".to_string(),
            },
        )
        .unwrap();

        // Decode and verify the claims
        let validation = Validation::new(Algorithm::HS256);
        let token_data =
            decode::<Claims>(&token, &DecodingKey::from_secret(jwt_secret()), &validation).unwrap();

        assert_eq!(token_data.claims.sub, user.id);
        assert_eq!(token_data.claims.username, "testuser");
        // Expiration should be roughly 30 days from now
        let now = chrono::Utc::now().timestamp() as usize;
        assert!(token_data.claims.exp > now);
        assert!(token_data.claims.exp <= now + 30 * 24 * 3600 + 60); // allow 60s tolerance
    }

    #[test]
    fn test_login_as_guest_creates_guest_user() {
        let (_tmp, conn) = setup_db();
        let (user, token) = login_as_guest(&conn).unwrap();

        assert_eq!(user.username, "guest");
        assert_eq!(user.email, "guest@local");
        assert!(!user.id.is_empty());
        assert!(!token.is_empty());

        // Verify token works
        let user_id = verify_token(&token).unwrap();
        assert_eq!(user_id, user.id);
    }

    #[test]
    fn test_login_as_guest_reuses_existing_guest() {
        let (_tmp, conn) = setup_db();
        let (user1, _) = login_as_guest(&conn).unwrap();
        let (user2, _) = login_as_guest(&conn).unwrap();

        assert_eq!(user1.id, user2.id);
        assert_eq!(user1.username, user2.username);
    }

    #[test]
    fn test_login_as_guest_does_not_conflict_with_regular_users() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);
        let (guest, _) = login_as_guest(&conn).unwrap();

        assert_ne!(guest.username, "testuser");
        assert_eq!(guest.username, "guest");
    }

    #[test]
    fn test_jwt_secret_from_env_uses_provided_value() {
        let secret = jwt_secret_from_env(Some("my-strong-secret".to_string()));
        assert_eq!(secret.as_slice(), b"my-strong-secret");
    }

    #[test]
    fn test_jwt_secret_from_env_falls_back_when_none() {
        let secret = jwt_secret_from_env(None);
        assert!(!secret.is_empty(), "ephemeral fallback must not be empty");
        assert!(
            secret.len() >= 32,
            "ephemeral fallback must be at least 32 bytes"
        );
    }

    #[test]
    fn test_jwt_secret_from_env_falls_back_when_empty_string() {
        let secret = jwt_secret_from_env(Some(String::new()));
        assert!(
            !secret.is_empty(),
            "empty env value should trigger fallback"
        );
        assert!(secret.len() >= 32);
    }

    #[test]
    fn test_jwt_secret_from_env_two_fallbacks_are_distinct() {
        let s1 = jwt_secret_from_env(None);
        let s2 = jwt_secret_from_env(None);
        assert_ne!(
            s1, s2,
            "each fallback call must generate a fresh random secret"
        );
    }
}
