use axum::{extract::Multipart, response::IntoResponse, http::StatusCode, Json};
use axum::extract::State;
use axum_extra::extract::SignedCookieJar;
use serde_json::json;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::{repository, AppState};
use crate::routes_utils::extract_user;

pub async fn upload_image_disk(
    State(state): State<AppState>,
    cookies: SignedCookieJar,
    mut multipart: Multipart
) -> impl IntoResponse {
    let user_id = extract_user(&cookies).unwrap();

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        if field.name() == Some("image") {
            // let filename = field.file_name().unwrap().to_string();
            let filename = Uuid::new_v4().to_string();

            // Ensure the uploads directory exists
            fs::create_dir_all(&state.config.image_upload_path).await.unwrap();

            let filepath = format!("{}/{}", &state.config.image_upload_path, filename);
            let mut file = File::create(&filepath).await.unwrap();

            while let Some(chunk) = field.chunk().await.unwrap() {
                file.write_all(&chunk).await.unwrap();
            }

            let image_id = repository::save_image(&state.db, &filename, user_id).await.unwrap();

            // Return the URL or file path of the uploaded image
            return Json(json!({
                "id": image_id,
                "filename": filename
            })).into_response()
        }
    }

    (StatusCode::BAD_REQUEST, "No image field found").into_response()
}