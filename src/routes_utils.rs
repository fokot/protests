use axum_extra::extract::SignedCookieJar;
use crate::localizer::for_language;

pub fn extract_language(cookies: &SignedCookieJar) -> (String, Box<dyn Fn(&str) -> String>) {
    let lang = cookies
        .get("language")
        .map(|c| c.value().to_string())
        .unwrap_or("sk".to_string());
    (lang.clone(), for_language(lang))
}