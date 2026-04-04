//! Phonetic generation service
//!
//! Pure-Rust implementations for romanization:
//! - Japanese: wana_kana (kana → romaji) — kanji are passed through unchanged
//! - Korean: algorithmic jamo decomposition → Revised Romanization
//! - French/English: returned unchanged (IPA requires python-phonetics feature)
//!
//! With the `python-phonetics` feature enabled, uses PyO3 bridges:
//! - Japanese: pykakasi (kanji → romaji)
//! - Korean: hangul-romanize
//! - French/English: epitran (text → IPA)

#[cfg(feature = "python-phonetics")]
use crate::Error;
use crate::Result;

/// Generate phonetic representation for lyrics
pub fn generate_phonetic(text: Vec<String>, language: &str) -> Result<Vec<String>> {
    match language {
        "jp" => japanese_to_romaji(text),
        "kr" => korean_to_roman(text),
        "fr" | "en" => to_ipa(text, language),
        _ => Ok(text),
    }
}

// ==================== JAPANESE ====================

#[cfg(feature = "python-phonetics")]
fn japanese_to_romaji(text: Vec<String>) -> Result<Vec<String>> {
    use pyo3::prelude::*;
    use pyo3::types::PyList;

    Python::with_gil(|py| {
        let kakasi_module = py.import("pykakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to import pykakasi: {}", e)))?;
        let kakasi = kakasi_module.getattr("kakasi")
            .map_err(|e| Error::Phonetic(format!("Failed to get kakasi class: {}", e)))?
            .call0()
            .map_err(|e| Error::Phonetic(format!("Failed to create kakasi instance: {}", e)))?;

        let mut result = Vec::new();
        for line in text {
            let converted = kakasi.call_method1("convert", (line,))
                .map_err(|e| Error::Phonetic(format!("kakasi.convert failed: {}", e)))?;
            let py_list: &PyList = converted.downcast()
                .map_err(|e| Error::Phonetic(format!("Failed to downcast to PyList: {}", e)))?;

            let mut romaji_line = String::new();
            for item in py_list {
                let dict = item.downcast::<pyo3::types::PyDict>()
                    .map_err(|e| Error::Phonetic(format!("Failed to downcast to PyDict: {}", e)))?;
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

#[cfg(not(feature = "python-phonetics"))]
fn japanese_to_romaji(text: Vec<String>) -> Result<Vec<String>> {
    use wana_kana::ConvertJapanese;
    // Pure-Rust: convert kana to romaji using wana_kana.
    // Kanji are passed through unchanged (full kanji support requires pykakasi).
    Ok(text.iter().map(|line| line.to_romaji()).collect())
}

// ==================== KOREAN ====================

#[cfg(feature = "python-phonetics")]
fn korean_to_roman(text: Vec<String>) -> Result<Vec<String>> {
    use pyo3::prelude::*;

    Python::with_gil(|py| {
        let module = py.import("hangul_romanize")
            .map_err(|e| Error::Phonetic(format!("Failed to import hangul_romanize: {}", e)))?;
        let transliter_class = module.getattr("Transliter")
            .map_err(|e| Error::Phonetic(format!("Failed to get Transliter class: {}", e)))?;

        let mut result = Vec::new();
        for line in text {
            let romanized = transliter_class.call1((line,))
                .map_err(|e| Error::Phonetic(format!("Transliter failed: {}", e)))?
                .extract::<String>()
                .map_err(|e| Error::Phonetic(format!("Failed to extract string: {}", e)))?;
            result.push(romanized);
        }
        Ok(result)
    })
}

#[cfg(not(feature = "python-phonetics"))]
fn korean_to_roman(text: Vec<String>) -> Result<Vec<String>> {
    // Pure-Rust: Revised Romanization via jamo decomposition
    Ok(text.iter().map(|line| romanize_korean(line)).collect())
}

#[cfg(not(feature = "python-phonetics"))]
fn romanize_korean(text: &str) -> String {
    // Hangul syllable block range: U+AC00 .. U+D7A3
    // Each block = (initial * 21 + medial) * 28 + final + 0xAC00
    const INITIALS: &[&str] = &[
        "g", "kk", "n", "d", "tt", "r", "m", "b", "pp", "s", "ss",
        "", "j", "jj", "ch", "k", "t", "p", "h",
    ];
    const MEDIALS: &[&str] = &[
        "a", "ae", "ya", "yae", "eo", "e", "yeo", "ye", "o", "wa", "wae",
        "oe", "yo", "u", "wo", "we", "wi", "yu", "eu", "ui", "i",
    ];
    const FINALS: &[&str] = &[
        "", "k", "k", "k", "n", "n", "n", "t", "l", "l", "l", "l", "l", "l", "l", "l",
        "m", "p", "p", "t", "t", "ng", "t", "t", "k", "t", "p", "t",
    ];

    let mut result = String::new();
    for c in text.chars() {
        let code = c as u32;
        if (0xAC00..=0xD7A3).contains(&code) {
            let offset = code - 0xAC00;
            let initial = (offset / 588) as usize;
            let medial = ((offset % 588) / 28) as usize;
            let final_c = (offset % 28) as usize;
            result.push_str(INITIALS[initial]);
            result.push_str(MEDIALS[medial]);
            result.push_str(FINALS[final_c]);
        } else {
            result.push(c);
        }
    }
    result
}

// ==================== FRENCH / ENGLISH (IPA) ====================

#[cfg(feature = "python-phonetics")]
fn to_ipa(text: Vec<String>, language: &str) -> Result<Vec<String>> {
    use pyo3::prelude::*;

    let lang_code = match language {
        "fr" => "fra-Latn",
        "en" => "eng-Latn",
        _ => return Ok(text),
    };

    Python::with_gil(|py| {
        let epitran_module = py.import("epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to import epitran: {}", e)))?;
        let epitran = epitran_module.getattr("Epitran")
            .map_err(|e| Error::Phonetic(format!("Failed to get Epitran class: {}", e)))?
            .call1((lang_code,))
            .map_err(|e| Error::Phonetic(format!("Failed to create Epitran instance: {}", e)))?;

        let mut result = Vec::new();
        for line in text {
            let ipa = epitran.call_method1("transliterate", (line,))
                .map_err(|e| Error::Phonetic(format!("transliterate failed: {}", e)))?
                .extract::<String>()
                .map_err(|e| Error::Phonetic(format!("Failed to extract string: {}", e)))?;
            result.push(ipa);
        }
        Ok(result)
    })
}

#[cfg(not(feature = "python-phonetics"))]
fn to_ipa(text: Vec<String>, _language: &str) -> Result<Vec<String>> {
    // IPA generation requires epitran (Python). Return original text.
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_phonetic_unsupported_language() {
        let text = vec!["Hello".to_string(), "World".to_string()];
        let result = generate_phonetic(text.clone(), "de").unwrap();
        assert_eq!(result, text);
    }

    #[test]
    fn test_french_english_returns_original_without_python() {
        // Without python-phonetics, FR/EN return original text
        let text = vec!["Bonjour".to_string()];
        let result = generate_phonetic(text.clone(), "fr").unwrap();
        #[cfg(not(feature = "python-phonetics"))]
        assert_eq!(result, text);
        #[cfg(feature = "python-phonetics")]
        assert!(!result[0].is_empty());
    }

    #[cfg(not(feature = "python-phonetics"))]
    #[test]
    fn test_japanese_kana_to_romaji() {
        let text = vec!["こんにちは".to_string()];
        let result = japanese_to_romaji(text).unwrap();
        assert_eq!(result[0], "konnichiha");
    }

    #[cfg(not(feature = "python-phonetics"))]
    #[test]
    fn test_korean_romanization() {
        let text = vec!["안녕하세요".to_string()];
        let result = korean_to_roman(text).unwrap();
        assert!(!result[0].is_empty());
        // Should produce something like "annyeonghaseyo"
        assert!(result[0].chars().all(|c| c.is_ascii() || c.is_whitespace()));
    }
}
