//! Phonetic generation service
//!
//! When the `python` feature is enabled, bridges to Python libraries for romanization:
//! - Japanese: pykakasi (kanji → romaji)
//! - Korean: hangul-romanize (hangul → latin)
//! - French/English: epitran (text → IPA)
//!
//! When the `python` feature is disabled (e.g. Android builds), phonetic generation
//! is unavailable and returns an error for supported languages.

use crate::{Error, Result};

/// Generate phonetic representation for lyrics
///
/// # Arguments
/// * `text` - Lines of lyrics to convert
/// * `language` - Language code: 'jp', 'kr', 'fr', 'en'
///
/// # Returns
/// Vector of phonetic representations matching input lines
#[cfg(feature = "python")]
pub fn generate_phonetic(text: &[String], language: &str) -> Result<Vec<String>> {
    // Short-circuit empty input so we don't spin up the Python interpreter
    // for a no-op.
    if text.is_empty() {
        return Ok(Vec::new());
    }
    match language {
        "jp" => japanese_to_romaji(text),
        "kr" => korean_to_roman(text),
        "fr" => to_ipa(text, "fra-Latn"),
        "en" => to_ipa(text, "eng-Latn"),
        _ => Ok(text.to_vec()), // Return original text for unsupported languages
    }
}

/// Stub implementation when Python is not available (e.g. Android)
#[cfg(not(feature = "python"))]
pub fn generate_phonetic(text: &[String], language: &str) -> Result<Vec<String>> {
    if text.is_empty() {
        return Ok(Vec::new());
    }
    match language {
        "jp" | "kr" | "fr" | "en" => Err(Error::Phonetic(
            "Phonetic generation is not available on this platform (requires Python)".to_string(),
        )),
        _ => Ok(text.to_vec()), // Return original text for unsupported languages
    }
}

// ===================== Python-backed implementations =====================

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyList;

/// Convert Japanese (including kanji) to romaji using pykakasi
#[cfg(feature = "python")]
fn japanese_to_romaji(text: &[String]) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import pykakasi module
        let kakasi_module = py
            .import("pykakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to import pykakasi: {}", e)))?;

        // Create kakasi instance
        let kakasi_class = kakasi_module
            .getattr("kakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to get kakasi class: {}", e)))?;
        let kakasi = kakasi_class
            .call0()
            .map_err(|e| Error::Phonetic(format!("Failed to create kakasi instance: {}", e)))?;

        let mut result = Vec::new();

        for line in text.iter() {
            // Convert each line
            let converted = kakasi
                .call_method1("convert", (line.as_str(),))
                .map_err(|e| Error::Phonetic(format!("kakasi.convert failed: {}", e)))?;

            // Extract romaji from each segment
            let py_list: &PyList = converted
                .downcast()
                .map_err(|e| Error::Phonetic(format!("Failed to downcast to PyList: {}", e)))?;

            let mut romaji_line = String::new();
            for item in py_list {
                let dict = item
                    .downcast::<pyo3::types::PyDict>()
                    .map_err(|e| Error::Phonetic(format!("Failed to downcast to PyDict: {}", e)))?;

                // Try 'hepburn' first, fallback to 'hira' if not available
                let romaji = if let Some(hepburn) = dict.get_item("hepburn")? {
                    hepburn.extract::<String>()?
                } else if let Some(hira) = dict.get_item("hira")? {
                    hira.extract::<String>()?
                } else {
                    dict.get_item("orig")?
                        .ok_or_else(|| Error::Phonetic("No romaji field found".to_string()))?
                        .extract::<String>()?
                };

                romaji_line.push_str(&romaji);
            }

            result.push(romaji_line);
        }

        Ok(result)
    })
}

/// Convert Korean hangul to romanized latin using hangul-romanize
#[cfg(feature = "python")]
fn korean_to_roman(text: &[String]) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import hangul_romanize module
        let module = py
            .import("hangul_romanize")
            .map_err(|e| Error::Phonetic(format!("Failed to import hangul_romanize: {}", e)))?;

        let transliter_class = module
            .getattr("Transliter")
            .map_err(|e| Error::Phonetic(format!("Failed to get Transliter class: {}", e)))?;

        let mut result = Vec::new();

        for line in text.iter() {
            let romanized = transliter_class
                .call1((line.as_str(),))
                .map_err(|e| Error::Phonetic(format!("Transliter failed: {}", e)))?
                .extract::<String>()
                .map_err(|e| Error::Phonetic(format!("Failed to extract string: {}", e)))?;

            result.push(romanized);
        }

        Ok(result)
    })
}

/// Convert text to IPA (International Phonetic Alphabet) using epitran
#[cfg(feature = "python")]
fn to_ipa(text: &[String], lang_code: &str) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import epitran module
        let epitran_module = py
            .import("epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to import epitran: {}", e)))?;

        let epitran_class = epitran_module
            .getattr("Epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to get Epitran class: {}", e)))?;

        // Create Epitran instance for the language
        let epitran = epitran_class
            .call1((lang_code,))
            .map_err(|e| Error::Phonetic(format!("Failed to create Epitran instance: {}", e)))?;

        let mut result = Vec::new();

        for line in text.iter() {
            let ipa = epitran
                .call_method1("transliterate", (line.as_str(),))
                .map_err(|e| Error::Phonetic(format!("transliterate failed: {}", e)))?
                .extract::<String>()
                .map_err(|e| Error::Phonetic(format!("Failed to extract string: {}", e)))?;

            result.push(ipa);
        }

        Ok(result)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phonetic_unsupported_language() {
        let text = vec!["Hello".to_string(), "World".to_string()];
        let result = generate_phonetic(&text, "de").unwrap();
        assert_eq!(result, text); // Should return original for unsupported language
    }

    #[test]
    fn test_generate_phonetic_empty_input() {
        // Empty input should return empty output across all branches without
        // touching Python at all.
        for lang in &["jp", "kr", "fr", "en", "de"] {
            let result = generate_phonetic(&[], lang).unwrap();
            assert!(
                result.is_empty(),
                "empty input must yield empty output for {}",
                lang
            );
        }
    }

    // ---- Stub branch: no `python` feature compiled in ----
    //
    // Verifies the Android/headless build path: a request for a supported
    // language must return a clear `Error::Phonetic` rather than silently
    // succeeding or panicking. Unsupported languages still pass through.
    #[test]
    #[cfg(not(feature = "python"))]
    fn test_stub_returns_error_for_supported_language() {
        let text = vec!["hello".to_string()];
        for lang in &["jp", "kr", "fr", "en"] {
            let result = generate_phonetic(&text, lang);
            assert!(
                matches!(result, Err(Error::Phonetic(_))),
                "stub must error for supported language {}, got {:?}",
                lang,
                result
            );
        }
    }

    #[test]
    #[cfg(not(feature = "python"))]
    fn test_stub_passthrough_for_unsupported_language() {
        let text = vec!["x".to_string()];
        let result = generate_phonetic(&text, "de").unwrap();
        assert_eq!(result, text);
    }

    // ---- Python-backed branch ----
    //
    // These tests require the Python interpreter and packages to be
    // available on the host:
    //     pip install pykakasi hangul-romanize epitran
    // They run with `cargo test --features python -- --ignored`. The CI
    // workflow (.github/workflows/ci-rust.yml) installs them and may
    // enable this set.

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_japanese_to_romaji() {
        let text = vec!["こんにちは".to_string()];
        let result = japanese_to_romaji(&text).unwrap();
        assert_eq!(result.len(), 1);
        assert!(
            !result[0].is_empty(),
            "kakasi must produce non-empty romaji"
        );
        assert!(
            result[0].chars().all(|c| c.is_ascii() || c.is_whitespace()),
            "romaji output should be ASCII, got {:?}",
            result[0]
        );
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_korean_to_roman() {
        let text = vec!["안녕하세요".to_string()];
        let result = korean_to_roman(&text).unwrap();
        assert_eq!(result.len(), 1);
        assert!(!result[0].is_empty());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_to_ipa_english() {
        let text = vec!["hello world".to_string()];
        let result = to_ipa(&text, "eng-Latn").unwrap();
        assert_eq!(result.len(), 1);
        assert!(!result[0].is_empty());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_to_ipa_french() {
        let text = vec!["bonjour le monde".to_string()];
        let result = to_ipa(&text, "fra-Latn").unwrap();
        assert_eq!(result.len(), 1);
        assert!(!result[0].is_empty());
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_generate_phonetic_dispatches_by_language() {
        // generate_phonetic must route to the right backend per language.
        // We don't assert on exact contents (depends on lib version), only
        // that each supported language yields a non-empty result with the
        // correct line count.
        let text = vec!["test line".to_string()];
        for lang in &["jp", "kr", "fr", "en"] {
            let result = generate_phonetic(&text, lang).expect("supported lang must succeed");
            assert_eq!(result.len(), 1, "{}: must keep line count", lang);
        }
    }
}
