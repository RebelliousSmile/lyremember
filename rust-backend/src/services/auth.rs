//! Authentication service

pub use crate::models::{User, LoginCredentials, RegisterData};

use crate::{Error, Result};
use rusqlite::Connection;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, Algorithm};
use serde::{Deserialize, Serialize};

const JWT_SECRET: &[u8] = b"your-secret-key-change-this-in-production"; // TODO: Use env var

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
        return Err(Error::Auth("Password must be at least 8 characters".to_string()));
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
         FROM users WHERE username = ?1"
    )?;
    
    let user = stmt.query_row([&credentials.username], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            genius_token: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|_| Error::Auth("Invalid username or password".to_string()))?;
    
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
        &EncodingKey::from_secret(JWT_SECRET),
    ).map_err(|e| Error::Auth(format!("Failed to generate token: {}", e)))?;
    
    Ok(token)
}

/// Verify JWT token and return user ID
pub fn verify_token(token: &str) -> Result<String> {
    let validation = Validation::new(Algorithm::HS256);
    
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &validation,
    ).map_err(|e| Error::Auth(format!("Invalid token: {}", e)))?;
    
    Ok(token_data.claims.sub)
}

/// Get user by ID
pub fn get_user_by_id(conn: &Connection, user_id: &str) -> Result<User> {
    let mut stmt = conn.prepare(
        "SELECT id, username, email, password_hash, genius_token, created_at
         FROM users WHERE id = ?1"
    )?;
    
    let user = stmt.query_row([user_id], |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
            email: row.get(2)?,
            password_hash: row.get(3)?,
            genius_token: row.get(4)?,
            created_at: row.get(5)?,
        })
    }).map_err(|_| Error::NotFound("User not found".to_string()))?;
    
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
        register(conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap()
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

        let result = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "other@example.com".to_string(),
            password: "password456".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username already taken"));
    }

    #[test]
    fn test_register_same_email_different_username_ok() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);

        // Same email but different username should succeed (no unique constraint on email)
        let result = register(&conn, RegisterData {
            username: "anotheruser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_empty_username() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username cannot be empty"));
    }

    #[test]
    fn test_register_whitespace_only_username() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "   ".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Username cannot be empty"));
    }

    #[test]
    fn test_register_empty_email() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "".to_string(),
            password: "password123".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Email cannot be empty"));
    }

    #[test]
    fn test_register_short_password() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "short".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Password must be at least 8 characters"));
    }

    #[test]
    fn test_register_password_exactly_8_chars() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "12345678".to_string(),
        });
        assert!(result.is_ok());
    }

    #[test]
    fn test_register_password_7_chars_fails() {
        let (_tmp, conn) = setup_db();
        let result = register(&conn, RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "1234567".to_string(),
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_login_wrong_password() {
        let (_tmp, conn) = setup_db();
        register_test_user(&conn);

        let result = login(&conn, LoginCredentials {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        });
        assert!(result.is_err());
        let err_msg = format!("{}", result.unwrap_err());
        assert!(err_msg.contains("Invalid username or password"));
    }

    #[test]
    fn test_login_nonexistent_user() {
        let (_tmp, conn) = setup_db();

        let result = login(&conn, LoginCredentials {
            username: "noone".to_string(),
            password: "password123".to_string(),
        });
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

        let user1 = register(&conn, RegisterData {
            username: "alice".to_string(),
            email: "alice@example.com".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        let user2 = register(&conn, RegisterData {
            username: "bob".to_string(),
            email: "bob@example.com".to_string(),
            password: "password456".to_string(),
        }).unwrap();

        assert_ne!(user1.id, user2.id);

        // Both can log in independently
        let (u1, _t1) = login(&conn, LoginCredentials {
            username: "alice".to_string(),
            password: "password123".to_string(),
        }).unwrap();
        assert_eq!(u1.id, user1.id);

        let (u2, _t2) = login(&conn, LoginCredentials {
            username: "bob".to_string(),
            password: "password456".to_string(),
        }).unwrap();
        assert_eq!(u2.id, user2.id);
    }

    #[test]
    fn test_token_contains_correct_claims() {
        let (_tmp, conn) = setup_db();
        let user = register_test_user(&conn);

        let (_, token) = login(&conn, LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        }).unwrap();

        // Decode and verify the claims
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret(JWT_SECRET),
            &validation,
        ).unwrap();

        assert_eq!(token_data.claims.sub, user.id);
        assert_eq!(token_data.claims.username, "testuser");
        // Expiration should be roughly 30 days from now
        let now = chrono::Utc::now().timestamp() as usize;
        assert!(token_data.claims.exp > now);
        assert!(token_data.claims.exp <= now + 30 * 24 * 3600 + 60); // allow 60s tolerance
    }
}
