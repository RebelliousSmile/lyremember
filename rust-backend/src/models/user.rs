//! User model

use serde::{Deserialize, Serialize};

/// User account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub genius_token: Option<String>,
    pub created_at: String,
}

impl User {
    /// Create a new user with generated UUID
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            username,
            email,
            password_hash,
            genius_token: None,
            created_at: chrono::Utc::now().to_rfc3339(),
        }
    }
}

/// User credentials for login
#[derive(Debug, Deserialize)]
pub struct LoginCredentials {
    pub username: String,
    pub password: String,
}

/// User registration data
#[derive(Debug, Clone, Deserialize)]
pub struct RegisterData {
    pub username: String,
    pub email: String,
    pub password: String,
}
