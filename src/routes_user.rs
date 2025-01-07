use axum::Form;
use axum::http::{HeaderMap, StatusCode};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::Cookie;
use axum_extra::extract::SignedCookieJar;
use sqlx::PgPool;
use time::Duration;
use crate::model::AuthData;
use crate::repository;

async fn login(
    Form(data): Form<AuthData>,
    db: PgPool,
    jar: SignedCookieJar,
) -> impl IntoResponse {
    let user = repository::login_user(&db, &data.username, &data.password)
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let mut cookie = Cookie::new("user_id", user.id.to_string());
    // FIXME TO SETTINGS
    cookie.set_max_age(Duration::days(30));
    let jar = jar.add(cookie);

    let mut headers = HeaderMap::new();
    headers.insert("Set-Cookie", jar.to_header().unwrap());

    Redirect::to("/").into_response();


    Ok((headers, Html(format!("Welcome, {}!", user.username))))
}