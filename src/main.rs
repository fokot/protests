mod localizer;
mod repository;
mod routes_protest;

use crate::localizer::for_language;
use crate::routes_protest::{
    add_protest, add_protest_form, delete_protest, edit_protest, edit_protest_form, list_protests,
};
use axum::{routing::get, Router};
use axum_extra::extract::cookie::CookieJar;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use tower_http::services::ServeDir;

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

fn extract_language(cookies: &CookieJar) -> (String, Box<dyn Fn(&str) -> String>) {
    let lang = cookies
        .get("language")
        .map(|c| c.value().to_string())
        .unwrap_or("sk".to_string());
    // return tuple (language, function)
    (lang.clone(), for_language(lang))
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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db_url.as_str())
        .await
        .unwrap();

    let state = AppState { db: pool.clone() };

    // sqlx::migrate!().run(&pool).await.unwrap();

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/", get(list_protests))
        .route("/protests", get(list_protests))
        .route("/protests/add", get(add_protest_form).post(add_protest))
        .route(
            "/protests/{id}/edit",
            get(edit_protest_form).post(edit_protest),
        )
        .route("/protests/{id}/delete", get(delete_protest))
        // server static files from assets directory
        // .nest("/assets", axum::service::get(axum::service::files::Files::new("assets")));
        .with_state(state);

    println!("Starting server on port {}", config.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
