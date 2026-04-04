//! LyRemember Backend Library
//! 
//! Core backend functionality for LyRemember lyrics memorization app.
//! Provides database management, authentication, translation, and phonetic services.

pub mod config;
pub mod db;
pub mod error;
pub mod models;
pub mod services;

// Re-export commonly used types
pub use error::{Error, Result};
pub use models::{Song, User, PracticeSession};
