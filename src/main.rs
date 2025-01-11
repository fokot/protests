mod localizer;
mod repository;
mod routes_protest;
mod routes_utils;
mod model;
mod routes_user;

use crate::routes_protest::{
    add_protest, add_protest_form, delete_protest, edit_protest, edit_protest_form, list_protests,
};
use axum::{routing::get, Router};
use axum::extract::FromRef;
use axum_extra::extract::cookie::Key;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use tower_http::services::ServeDir;
use crate::routes_user::{change_language, login_generate_code, login_with_code};

#[derive(Clone)]
struct AppState {
    db: PgPool,
    web_key: Key,
    config: Config,
}

#[derive(Debug, Deserialize, Clone)]
struct Config {
    port: u16,
    db_url: String,
    web_key: String,
    login_expiration_days: i32,
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

// this impl tells `SignedCookieJar` how to access the key from our state
impl FromRef<AppState> for Key {
    fn from_ref(state: &AppState) -> Self {
        state.web_key.clone()
    }
}

// Main function
#[tokio::main]
async fn main() {
    let config = load_config();
    let pool = create_db_pool(&config).await;
    let web_key = Key::from(config.web_key.as_bytes());
    let state = AppState { db: pool.clone(), web_key, config: config.clone() };
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
        .route("/login/generate-code/{email}", get(login_generate_code))
        .route("/login/code/{id}/{code}", get(login_with_code))
        .route("/change-language/{code}", get(change_language))
        // server static files from assets directory
        // .nest("/assets", axum::service::get(axum::service::files::Files::new("assets")));
        .with_state(state);

    println!("Starting server on port {}", config.port);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
