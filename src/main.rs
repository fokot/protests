mod localizer;
mod repository;
mod routes_protest;
mod routes_utils;

use crate::routes_protest::{
    add_protest, add_protest_form, delete_protest, edit_protest, edit_protest_form, list_protests,
};
use axum::{routing::get, Router};
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

#[derive(Clone)]
struct AppState {
    db: PgPool,
}

#[derive(Debug, Deserialize)]
struct Config {
    port: u16,
    db_url: String,
}

fn load_config() -> Config {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("Settings"))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
        .unwrap();

    settings.try_deserialize::<Config>().unwrap()
}

async fn create_db_pool(config: &Config) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(config.db_url.as_str())
        .await
        .unwrap()
}

// Main function
#[tokio::main]
async fn main() {
    let config = load_config();
    let pool = create_db_pool(&config).await;
    let state = AppState { db: pool.clone() };
    sqlx::migrate!().run(&pool).await.unwrap();

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
