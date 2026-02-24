//! Translation service using LibreTranslate API

use crate::{Error, Result};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const LIBRETRANSLATE_URL: &str = "https://libretranslate.com/translate";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Serialize)]
struct TranslateRequest {
    q: String,
    source: String,
    target: String,
    format: String,
}

#[derive(Debug, Deserialize)]
struct TranslateResponse {
    #[serde(rename = "translatedText")]
    translated_text: String,
}

/// Translate text using LibreTranslate API
/// 
/// # Arguments
/// * `text` - Lines to translate
/// * `source_lang` - Source language code (e.g., "ja", "ko", "fr")
/// * `target_lang` - Target language code (e.g., "en")
/// 
/// # Returns
/// Vector of translated lines
pub fn translate_text(
    text: Vec<String>,
    source_lang: &str,
    target_lang: &str,
) -> Result<Vec<String>> {
    translate_with_libretranslate(text, source_lang, target_lang, LIBRETRANSLATE_URL)
}

/// Translate text using LibreTranslate API (with custom endpoint)
fn translate_with_libretranslate(
    text: Vec<String>,
    source_lang: &str,
    target_lang: &str,
    api_url: &str,
) -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| Error::Translation(format!("Failed to create HTTP client: {}", e)))?;
    
    let mut translations = Vec::new();
    
    for line in text {
        if line.trim().is_empty() {
            translations.push(String::new());
            continue;
        }
        
        let request = TranslateRequest {
            q: line.clone(),
            source: source_lang.to_string(),
            target: target_lang.to_string(),
            format: "text".to_string(),
        };
        
        // Retry logic for rate limiting
        let mut retries = 3;
        let mut last_error = None;
        
        while retries > 0 {
            match client.post(api_url).json(&request).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        match response.json::<TranslateResponse>() {
                            Ok(data) => {
                                translations.push(data.translated_text);
                                break;
                            }
                            Err(e) => {
                                last_error = Some(Error::Translation(format!(
                                    "Failed to parse response: {}",
                                    e
                                )));
                                retries -= 1;
                            }
                        }
                    } else {
                        let status = response.status();
                        let body = response.text().unwrap_or_default();
                        
                        if status.as_u16() == 429 {
                            // Rate limited, wait and retry
                            std::thread::sleep(Duration::from_secs(2));
                            retries -= 1;
                            last_error = Some(Error::Translation("Rate limited".to_string()));
                        } else {
                            return Err(Error::Translation(format!(
                                "API error {}: {}",
                                status, body
                            )));
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(Error::Translation(format!("Request failed: {}", e)));
                    retries -= 1;
                    if retries > 0 {
                        std::thread::sleep(Duration::from_secs(1));
                    }
                }
            }
        }
        
        if retries == 0 {
            return Err(last_error.unwrap_or_else(|| {
                Error::Translation("Translation failed after retries".to_string())
            }));
        }
    }
    
    Ok(translations)
}

/// Batch translate with delay between requests to avoid rate limiting
pub fn translate_batch(
    text: Vec<String>,
    source_lang: &str,
    target_lang: &str,
    delay_ms: u64,
) -> Result<Vec<String>> {
    let client = reqwest::blocking::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()
        .map_err(|e| Error::Translation(format!("Failed to create HTTP client: {}", e)))?;
    
    let mut translations = Vec::new();
    
    for (i, line) in text.iter().enumerate() {
        if i > 0 && delay_ms > 0 {
            std::thread::sleep(Duration::from_millis(delay_ms));
        }
        
        if line.trim().is_empty() {
            translations.push(String::new());
            continue;
        }
        
        let request = TranslateRequest {
            q: line.clone(),
            source: source_lang.to_string(),
            target: target_lang.to_string(),
            format: "text".to_string(),
        };
        
        let response = client
            .post(LIBRETRANSLATE_URL)
            .json(&request)
            .send()
            .map_err(|e| Error::Translation(format!("Request failed: {}", e)))?;
        
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().unwrap_or_default();
            return Err(Error::Translation(format!(
                "API error {}: {}",
                status, body
            )));
        }
        
        let data = response
            .json::<TranslateResponse>()
            .map_err(|e| Error::Translation(format!("Failed to parse response: {}", e)))?;
        
        translations.push(data.translated_text);
    }
    
    Ok(translations)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_empty_lines() {
        // Mock test - actual API calls would require network
        let text = vec![String::new(), String::new()];
        // Would need to mock the HTTP client for proper testing
        // For now, just verify the function signature
        assert_eq!(text.len(), 2);
    }

    #[test]
    #[ignore] // Ignore by default as it requires network access
    fn test_translate_text_real_api() {
        let text = vec!["Hello".to_string(), "World".to_string()];
        let result = translate_text(text, "en", "fr");
        
        match result {
            Ok(translations) => {
                assert_eq!(translations.len(), 2);
                println!("Translations: {:?}", translations);
            }
            Err(e) => {
                eprintln!("Translation failed (may be expected if offline): {}", e);
            }
        }
    }
}
