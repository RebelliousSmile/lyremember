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
pub fn generate_phonetic(text: Vec<String>, language: &str) -> Result<Vec<String>> {
    match language {
        "jp" => japanese_to_romaji(text),
        "kr" => korean_to_roman(text),
        "fr" => to_ipa(text, "fra-Latn"),
        "en" => to_ipa(text, "eng-Latn"),
        _ => Ok(text), // Return original text for unsupported languages
    }
}

/// Stub implementation when Python is not available (e.g. Android)
#[cfg(not(feature = "python"))]
pub fn generate_phonetic(_text: Vec<String>, language: &str) -> Result<Vec<String>> {
    match language {
        "jp" | "kr" | "fr" | "en" => Err(Error::Phonetic(
            "Phonetic generation is not available on this platform (requires Python)".to_string(),
        )),
        _ => Ok(_text), // Return original text for unsupported languages
    }
}

// ===================== Python-backed implementations =====================

#[cfg(feature = "python")]
use pyo3::prelude::*;
#[cfg(feature = "python")]
use pyo3::types::PyList;

/// Convert Japanese (including kanji) to romaji using pykakasi
#[cfg(feature = "python")]
fn japanese_to_romaji(text: Vec<String>) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import pykakasi module
        let kakasi_module = py.import("pykakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to import pykakasi: {}", e)))?;

        // Create kakasi instance
        let kakasi_class = kakasi_module.getattr("kakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to get kakasi class: {}", e)))?;
        let kakasi = kakasi_class.call0()
            .map_err(|e| Error::Phonetic(format!("Failed to create kakasi instance: {}", e)))?;

        let mut result = Vec::new();

        for line in text {
            // Convert each line
            let converted = kakasi
                .call_method1("convert", (line,))
                .map_err(|e| Error::Phonetic(format!("kakasi.convert failed: {}", e)))?;

            // Extract romaji from each segment
            let py_list: &PyList = converted.downcast()
                .map_err(|e| Error::Phonetic(format!("Failed to downcast to PyList: {}", e)))?;

            let mut romaji_line = String::new();
            for item in py_list {
                let dict = item.downcast::<pyo3::types::PyDict>()
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
fn korean_to_roman(text: Vec<String>) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import hangul_romanize module
        let module = py.import("hangul_romanize")
            .map_err(|e| Error::Phonetic(format!("Failed to import hangul_romanize: {}", e)))?;

        let transliter_class = module.getattr("Transliter")
            .map_err(|e| Error::Phonetic(format!("Failed to get Transliter class: {}", e)))?;

        let mut result = Vec::new();

        for line in text {
            let romanized = transliter_class
                .call1((line,))
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
fn to_ipa(text: Vec<String>, lang_code: &str) -> Result<Vec<String>> {
    Python::with_gil(|py| {
        // Import epitran module
        let epitran_module = py.import("epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to import epitran: {}", e)))?;

        let epitran_class = epitran_module.getattr("Epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to get Epitran class: {}", e)))?;

        // Create Epitran instance for the language
        let epitran = epitran_class.call1((lang_code,))
            .map_err(|e| Error::Phonetic(format!("Failed to create Epitran instance: {}", e)))?;

        let mut result = Vec::new();

        for line in text {
            let ipa = epitran
                .call_method1("transliterate", (line,))
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
        let result = generate_phonetic(text.clone(), "de").unwrap();
        assert_eq!(result, text); // Should return original for unsupported language
    }

    // Note: The following tests require Python packages to be installed:
    // pip install pykakasi hangul-romanize epitran

    #[test]
    #[ignore] // Ignore by default, run with `cargo test -- --ignored` if Python deps are installed
    #[cfg(feature = "python")]
    fn test_japanese_to_romaji() {
        let text = vec!["こんにちは".to_string()];
        let result = japanese_to_romaji(text).unwrap();
        assert!(!result[0].is_empty());
        println!("Japanese romaji: {:?}", result);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_korean_to_roman() {
        let text = vec!["안녕하세요".to_string()];
        let result = korean_to_roman(text).unwrap();
        assert!(!result[0].is_empty());
        println!("Korean roman: {:?}", result);
    }

    #[test]
    #[ignore]
    #[cfg(feature = "python")]
    fn test_to_ipa_english() {
        let text = vec!["hello world".to_string()];
        let result = to_ipa(text, "eng-Latn").unwrap();
        assert!(!result[0].is_empty());
        println!("English IPA: {:?}", result);
    }
}
