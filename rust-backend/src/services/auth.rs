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

    #[test]
    fn test_register_and_login() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        // Register user
        let register_data = RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        let user = register(&conn, register_data).unwrap();
        assert_eq!(user.username, "testuser");
        
        // Login
        let credentials = LoginCredentials {
            username: "testuser".to_string(),
            password: "password123".to_string(),
        };
        
        let (logged_user, token) = login(&conn, credentials).unwrap();
        assert_eq!(logged_user.id, user.id);
        assert!(!token.is_empty());
        
        // Verify token
        let user_id = verify_token(&token).unwrap();
        assert_eq!(user_id, user.id);
    }

    #[test]
    fn test_register_duplicate_username() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        let register_data = RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        register(&conn, register_data.clone()).unwrap();
        
        // Try to register again with same username
        let result = register(&conn, register_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_login_wrong_password() {
        let temp_file = NamedTempFile::new().unwrap();
        let conn = init_database(temp_file.path()).unwrap();
        
        let register_data = RegisterData {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "password123".to_string(),
        };
        
        register(&conn, register_data).unwrap();
        
        // Try to login with wrong password
        let credentials = LoginCredentials {
            username: "testuser".to_string(),
            password: "wrongpassword".to_string(),
        };
        
        let result = login(&conn, credentials);
        assert!(result.is_err());
    }
}
