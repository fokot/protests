use axum_extra::extract::CookieJar;
use crate::localizer::for_language;

pub fn extract_language(cookies: &CookieJar) -> (String, Box<dyn Fn(&str) -> String>) {
    let lang = cookies
        .get("language")
        .map(|c| c.value().to_string())
        .unwrap_or("sk".to_string());
    // return tuple (language, function)
    (lang.clone(), for_language(lang))
}