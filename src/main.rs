use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Router};
use db::establish_connection;
use handler::HiAnime;
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use tokio::sync::Mutex;

mod db;
mod error;
mod handler;
mod model;
mod query;

// Catch-all handler to redirect other routes to /v1/health
async fn not_found() -> impl IntoResponse {
    StatusCode::NOT_FOUND
}

// health check
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Retrieve the database URL from secrets
    let secret = secret_store.get("DATABASE_URL").unwrap();

    // Retrieve the ca certoficate from secrets
    let cert = secret_store.get("CA_CERT_BASE64").unwrap();

    // Establish the database connection and wrap it in an Arc for shared ownership
    let client = Arc::new(establish_connection(&secret, &cert).await.unwrap());

    let hianime_cache = Arc::new(Mutex::new(HiAnime::new()));

    // Setup router with routes and the database client
    let router = Router::new()
        .route("/v1/health", get(health_check))
        .route("/v1/list", get(HiAnime::get_anime_ids_handler))
        .route(
            "/v1/anime/{anime_id}",
            get(HiAnime::get_anime_detail_by_id_handler),
        )
        .route(
            "/v1/anime/{anime_id}/episodes",
            get(HiAnime::get_anime_episodes_info_handler),
        )
        .route(
            "/v1/anime/{anime_id}/staff",
            get(HiAnime::get_anime_staff_info_handler),
        )
        .route("/{*wildcard}", get(not_found))
        .layer(Extension(hianime_cache))
        .layer(Extension(client));

    Ok(router.into())
}
