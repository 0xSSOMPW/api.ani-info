use axum::{extract::Extension, http::StatusCode, response::IntoResponse, routing::get, Router};
use db::establish_connection;
use handler::{
    get_anime_detail_by_id_handler, get_anime_episodes_info_handler, get_anime_ids_handler,
    get_anime_staff_info_handler,
};
use shuttle_runtime::SecretStore;
use std::sync::Arc;

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

    // Establish the database connection and wrap it in an Arc for shared ownership
    let client = Arc::new(establish_connection(&secret).await.unwrap());

    // Setup router with routes and the database client
    let router = Router::new()
        .route("/v1/health", get(health_check))
        .route("/v1/list", get(get_anime_ids_handler))
        .route("/v1/anime/:anime_id", get(get_anime_detail_by_id_handler))
        .route(
            "/v1/anime/:anime_id/episodes",
            get(get_anime_episodes_info_handler),
        )
        .route(
            "/v1/anime/:anime_id/staff",
            get(get_anime_staff_info_handler),
        )
        .route("/*path", get(not_found))
        .layer(Extension(client));

    Ok(router.into())
}
