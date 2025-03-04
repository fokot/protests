use axum_extra::extract::SignedCookieJar;
use crate::localizer::{for_language, LocalizationFn};

pub fn extract_language(cookies: &SignedCookieJar) -> (String, LocalizationFn) {
    let lang = cookies
        .get("language")
        .map(|c| c.value().to_string())
        .unwrap_or("sk".to_string());
    (lang.clone(), for_language(lang))
}

// pub fn language_icon(lang: String) {
//     match lang.as_str() {
//         "sk" => "🇸🇰",
//         "en" => "🇬🇧",
//         _ => "🌍"
//     }
// }

pub fn all_languages() -> Vec<String> {
    vec!["sk".to_string(), "en".to_string()]
}

pub fn extract_user(cookies: &SignedCookieJar) -> Option<i32> {
    cookies
        .get("user_id")
        .and_then(|c| c.value().parse().ok())
}