use askama::Template;
use axum::extract::{Path, Query, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::SignedCookieJar;
use crate::{repository, AppState};
use crate::localizer::{for_language, LocalizationFn};
use crate::routes_utils::{extract_language, extract_user};
use crate::model::{Protest, ProtestSave, ProtestSearch};

#[derive(Template)]
#[template(path = "protests.html")]
struct ProtestsTemplate {
    protests: Vec<Protest>,
    tags: Vec<String>,
    m: LocalizationFn,
    lang: String,
    user_id: String,
}

pub async fn list_protests(
    State(state): State<AppState>,
    cookies: SignedCookieJar,
    Query(search): Query<ProtestSearch>,
) -> Html<String> {
    // FIXME when using extract_cookie it fails with
    // error[E0277]: the trait bound `fn(State<AppState>, SignedCookieJar) -> impl Future<Output = impl IntoResponse> {list_protests}: Handler<_, _>` is not satisfied
    let l = cookies.get("language").map(|c| c.value().to_string()).unwrap_or("sk".to_string());
    let lang = l.clone();

    let user_id = extract_user(&cookies).map(|id| id.to_string()).unwrap_or("NOT LOGGED IN".to_string());

    let protests = repository::list_protests(&state.db, search).await.unwrap();

    let template = ProtestsTemplate { protests, tags: Vec::new(), m: for_language(l), lang, user_id };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "protest_add.html")]
struct ProtestAddTemplate {
    lang: String,
    m: LocalizationFn,
}

pub async fn add_protest_form(cookies: SignedCookieJar) -> impl IntoResponse {
    let (lang, m) = extract_language(&cookies);
    let template = ProtestAddTemplate { lang, m } ;
    Html(template.render().unwrap()).into_response()
}

pub async fn add_protest(
    State(state): State<AppState>,
    Form(protest): Form<ProtestSave>,
) -> impl IntoResponse {
    let result = repository::create_protest(&state.db, &protest).await;

    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add protest: {}", err),
        )
            .into_response(),
    }
}

#[derive(Template)]
#[template(path = "protest_edit.html")]
struct ProtestEditTemplate {
    protest: Protest,
    lang: String,
    m: LocalizationFn,
}

pub async fn edit_protest_form(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
    cookies: SignedCookieJar,
) -> impl IntoResponse {
    // Fetch the protest from the database
    let protest = repository::get_protest(&state.db, protest_id).await;

    match protest {
        Ok(protest) => {
            let (lang, m) = extract_language(&cookies);
            let template = ProtestEditTemplate { protest, lang, m };
            Html(template.render().unwrap()).into_response()
        }
        Err(_) => (
            StatusCode::NOT_FOUND,
            "Protest not found".to_string(),
        )
            .into_response(),
    }
}

pub async fn edit_protest(
    State(state): State<AppState>,
    Form(protest): Form<Protest>,
) -> impl IntoResponse {
    // Update the protest in the database
    let result = repository::edit_protest(&state.db, &protest).await;

    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update protest: {}", err),
        )
            .into_response(),
    }
}

pub async fn delete_protest(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
) -> impl IntoResponse {
    let result = repository::delete_protest(&state.db, protest_id).await;
    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete protest: {}", err),
        )
            .into_response(),
    }
}