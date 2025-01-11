use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::SignedCookieJar;
use time::Duration;
use uuid::Uuid;
use crate::{repository, AppState};

pub async fn login_generate_code(
    Path(email): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {

    let code = Uuid::new_v4();
    let id = repository::save_login_code(&state.db, email.as_str(), code.to_string().as_str()).await.unwrap();
    format!("/login/code/{}/{}", id, code).to_string().into_response()
}

pub async fn login_with_code(
    Path((id, code)): Path<(i32, String)>,
    State(state): State<AppState>,
    jar: SignedCookieJar,
) -> impl IntoResponse {
    let res = repository::check_login_code(&state.db, id, &code, state.config.login_expiration_days).await;

    match res {
        Ok(Some(_)) => {
            let mut cookie = Cookie::new("user_id", id.to_string());
            cookie.set_max_age(Duration::days(30));
            let jar = jar.add(cookie);
            (jar, Redirect::to("/")).into_response()
        },
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn change_language(
    Path(code): Path<String>,
    jar: SignedCookieJar,
) -> impl IntoResponse {
    let mut cookie = Cookie::new("language", code.to_string());
    cookie.set_max_age(Duration::days(30));
    cookie.set_path("/");
    let jar = jar.add(cookie);
    jar.into_response()
}