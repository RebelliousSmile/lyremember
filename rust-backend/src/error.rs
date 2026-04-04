//! Error types for LyRemember backend

use thiserror::Error;

/// Result type alias for LyRemember operations
pub type Result<T> = std::result::Result<T, Error>;

/// Main error type for LyRemember backend
#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Python error: {0}")]
    Python(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Translation error: {0}")]
    Translation(String),

    #[error("Phonetic generation error: {0}")]
    Phonetic(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

#[cfg(feature = "python-phonetics")]
impl From<pyo3::PyErr> for Error {
    fn from(err: pyo3::PyErr) -> Self {
        Error::Python(err.to_string())
    }
}
