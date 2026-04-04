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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::Auth("bad token".to_string());
        assert_eq!(format!("{}", err), "Authentication error: bad token");

        let err = Error::NotFound("song".to_string());
        assert_eq!(format!("{}", err), "Not found: song");

        let err = Error::Translation("timeout".to_string());
        assert_eq!(format!("{}", err), "Translation error: timeout");

        let err = Error::Phonetic("unsupported".to_string());
        assert_eq!(format!("{}", err), "Phonetic generation error: unsupported");
    }
}

#[cfg(feature = "python")]
impl From<pyo3::PyErr> for Error {
    fn from(err: pyo3::PyErr) -> Self {
        Error::Python(err.to_string())
    }
}
