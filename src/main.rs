mod localizer;

use askama::Template;
use axum::{response::Html, routing::get, Form, Router};
use serde::Deserialize;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum_extra::extract::cookie::CookieJar;
use axum::response::{IntoResponse, Redirect};
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::localizer::for_language;

// Define a Protest structure for deserialization
#[derive(Clone, Debug, Deserialize, FromRow)]
struct Protest {
    id: i32,
    name: String,
    description: String,
    labels: String,
    town: String,
    region: String,
    country: String,
    date: String,
    time: String,
    place: String,
}

// Askama template for the list of protests
#[derive(Template)]
#[template(path = "protests.html")]
struct ProtestsTemplate {
    protests: Vec<Protest>,
    m: Box<dyn Fn(&str) -> String>,
    lang: String
}

fn extract_language(cookies: &CookieJar) -> (String, Box<dyn Fn(&str) -> String>) {
    let lang = cookies.get("language").map(|c| c.value().to_string()).unwrap_or("sk".to_string());
    // return tuple (language, function)
    (lang.clone(), for_language(lang))
}

async fn list_protests(
    State(state): State<AppState>,
    cookies: CookieJar
) -> Html<String> {
    // FIXME when using extract_cookie it fails with
    // error[E0277]: the trait bound `fn(State<AppState>, CookieJar) -> impl Future<Output = impl IntoResponse> {list_protests}: Handler<_, _>` is not satisfied
    let l = cookies.get("language").map(|c| c.value().to_string()).unwrap_or("sk".to_string());
    let lang = l.clone();

    let protests = sqlx::query_as::<_, Protest>(
        "SELECT id, name, description, labels, town, region, country, date, time, place FROM protests ORDER BY id"
    )
    .fetch_all(&state.db).await.unwrap();

    let template = ProtestsTemplate { protests, m: for_language(l), lang };
    Html(template.render().unwrap())
}

#[derive(Template)]
#[template(path = "protest_add.html")]
pub struct ProtestAddTemplate {
    lang: String,
    m: Box<dyn Fn(&str) -> String>,
}

async fn add_protest_form(cookies: CookieJar) -> impl IntoResponse {
    let (lang, m) = extract_language(&cookies);
    let template = ProtestAddTemplate { lang, m } ;
    Html(template.render().unwrap()).into_response()
}

#[derive(Template)]
#[template(path = "protest_edit.html")]
pub struct ProtestEditTemplate {
    protest: Protest,
    lang: String,
    m: Box<dyn Fn(&str) -> String>,
}

async fn edit_protest_form(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
    cookies: CookieJar,
) -> impl IntoResponse {
    // Fetch the protest from the database
    let protest = sqlx::query_as::<_, Protest>(
        "SELECT id, name, description, labels, town, region, country, date, time, place FROM protests WHERE id = $1"
    )
        .bind(protest_id)
        .fetch_one(&state.db)
        .await;

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


// Route to handle form submission and add protest to the database
async fn add_protest(
    State(state): State<AppState>,
    Form(protest): Form<Protest>,
) -> impl IntoResponse {
    let result = sqlx::query(
        "INSERT INTO protests (name, description, labels, town, region, country, date, time, place)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
        .bind(&protest.name)
        .bind(&protest.description)
        .bind(&protest.labels)
        .bind(&protest.town)
        .bind(&protest.region)
        .bind(&protest.country)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.place)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to add protest: {}", err),
        )
            .into_response(),
    }
}

async fn edit_protest(
    State(state): State<AppState>,
    Form(protest): Form<Protest>,
) -> impl IntoResponse {
    // Update the protest in the database
    let result = sqlx::query(
        r#"UPDATE protests SET
            name = $1,
            description = $2,
            labels = $3,
            town = $4,
            region = $5,
            country = $6,
            date = $7,
            time = $8,
            place = $9
            WHERE id = $10"#
    )
        .bind(&protest.name)
        .bind(&protest.description)
        .bind(&protest.labels)
        .bind(&protest.town)
        .bind(&protest.region)
        .bind(&protest.country)
        .bind(&protest.date)
        .bind(&protest.time)
        .bind(&protest.place)
        .bind(protest.id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to update protest: {}", err),
        )
            .into_response(),
    }
}

async fn delete_protest(
    State(state): State<AppState>,
    Path(protest_id): Path<i32>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM protests WHERE id = $1")
        .bind(protest_id)
        .execute(&state.db)
        .await;

    match result {
        Ok(_) => Redirect::to("/protests").into_response(),
        Err(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to delete protest: {}", err),
        )
            .into_response(),
    }
}

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    db_url: String,
}

// Main function
#[tokio::main]
async fn main() {

    let settings = config::Config::builder()
        .add_source(config::File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    let config = settings.try_deserialize::<Config>().unwrap();

    let pool =
        PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db_url.as_str()).await.unwrap();

    let state = AppState { db: pool.clone() };

    sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .route("/", get(list_protests))
        .route("/protests", get(list_protests))
        .route("/protests/add", get(add_protest_form).post(add_protest))
        .route("/protests/{id}/edit", get(edit_protest_form).post(edit_protest))
        .route("/protests/{id}/delete", get(delete_protest))
        .with_state(state);

    println!("Starting server on port {}", config.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
