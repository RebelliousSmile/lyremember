//! Application configuration

use std::env;

/// Default LibreTranslate API endpoint
const DEFAULT_LIBRETRANSLATE_URL: &str = "https://libretranslate.com/translate";

/// Get LibreTranslate API URL from environment or default
pub fn libretranslate_url() -> String {
    env::var("LYREMEMBER_LIBRETRANSLATE_URL")
        .unwrap_or_else(|_| DEFAULT_LIBRETRANSLATE_URL.to_string())
}
