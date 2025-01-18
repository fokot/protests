use std::collections::HashMap;
use askama::Template;
use axum::extract::{Path, Query, State};
use axum::Form;
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::SignedCookieJar;
use crate::{repository, AppState};
use crate::localizer::{for_language, LocalizationFn};
use crate::routes_utils::{extract_language, extract_user};
use crate::model::{Protest, ProtestSave, ProtestSearch, Region};

#[derive(Template)]
#[template(path = "protests.html")]
struct ProtestsTemplate {
    regions: (Vec<Region>, Vec<(String, Vec<Region>)>),
    protests: Vec<Protest>,
    tags: Vec<String>,
    m: LocalizationFn,
    lang: String,
    user_id: Option<i32>,
}



fn group_and_sort_regions(
    regions: Vec<Region>,
) -> (Vec<Region>, Vec<(String, Vec<Region>)>) {

    let mut parent_regions: Vec<Region> = regions
        .iter()
        .filter(|region| region.parent_id.is_none())
        .map(|region| region.clone())
        .collect();

    // Map to store top-level region names by their ID
    let parent_name_map: HashMap<i32, String> = parent_regions
        .iter()
        .map(|region| (region.id, region.name.clone()))
        .collect();

    let mut grouped: HashMap<String, Vec<Region>> = HashMap::new();

    // Group regions by parent name
    for region in regions {
        if let Some(parent_id) = region.parent_id {
            let parent_name = parent_name_map.get(&parent_id).cloned().unwrap();
            grouped.entry(parent_name).or_insert_with(Vec::new).push(region);
        }
    }

    // Collect the groups into a Vec and sort each group by name
    let mut sub_regions: Vec<(String, Vec<Region>)> = grouped
        .into_iter()
        .map(|(parent_name, mut group)| {
            group.sort_by(|a, b| a.name.cmp(&b.name));
            (parent_name, group)
        })
        .collect();

    // Sort the outer Vec by parent name
    sub_regions.sort_by(|a, b| a.0.cmp(&b.0));

    (parent_regions, sub_regions)
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


    let user_id = extract_user(&cookies);
    let protests = repository::list_protests(&state.db, search).await.unwrap();

    let regions = group_and_sort_regions(repository::list_regions(&state.db).await.unwrap());

    let template = ProtestsTemplate { regions, protests, tags: Vec::new(), m: for_language(l), lang, user_id };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "protest_view.html")]
struct ProtestViewTemplate {
    protest: Protest,
    lang: String,
    m: LocalizationFn,
    user_id: Option<i32>,
}

pub async fn view_protest(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
    cookies: SignedCookieJar,
) -> impl IntoResponse {
    // Fetch the protest from the database
    let protest = repository::get_protest(&state.db, protest_id).await;
    let user_id = extract_user(&cookies);
    match protest {
        Ok(protest) => {
            let (lang, m) = extract_language(&cookies);
            let template = ProtestViewTemplate { protest, lang, m, user_id };
            Html(template.render().unwrap()).into_response()
        }
        Err(err) => (
            StatusCode::NOT_FOUND,
            format!("Protest not found {}", err.to_string()).to_string(),
        )
            .into_response(),
    }
}

#[derive(Template)]
#[template(path = "protest_add.html")]
struct ProtestAddTemplate {
    lang: String,
    m: LocalizationFn,
    user_id: Option<i32>,
}

pub async fn add_protest_form(cookies: SignedCookieJar) -> impl IntoResponse {
    let (lang, m) = extract_language(&cookies);
    let user_id = extract_user(&cookies);
    let template = ProtestAddTemplate { lang, m, user_id } ;
    Html(template.render().unwrap()).into_response()
}

pub async fn add_protest(
    State(state): State<AppState>,
    cookies: SignedCookieJar,
    Form(protest): Form<ProtestSave>,
) -> impl IntoResponse {

    let user_id = extract_user(&cookies).unwrap();
    let result = repository::create_protest(&state.db, &protest, user_id).await;

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
    user_id: Option<i32>,
}

pub async fn edit_protest_form(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
    cookies: SignedCookieJar,
) -> impl IntoResponse {
    // Fetch the protest from the database
    let protest = repository::get_protest(&state.db, protest_id).await;
    let user_id = extract_user(&cookies);

    match protest {
        Ok(protest) => {
            let (lang, m) = extract_language(&cookies);
            let template = ProtestEditTemplate { protest, lang, m, user_id };
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