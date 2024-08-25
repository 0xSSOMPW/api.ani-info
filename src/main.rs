use axum::{extract::Extension, routing::get, Router};
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

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    // Retrieve the database URL from secrets
    let secret = secret_store.get("DATABASE_URL").unwrap();

    // Establish the database connection and wrap it in an Arc for shared ownership
    let client = Arc::new(establish_connection(secret).await.unwrap());

    // Setup router with routes and the database client
    let router = Router::new()
        .route("/list", get(get_anime_ids_handler))
        .route("/anime/:anime_id", get(get_anime_detail_by_id_handler))
        .route(
            "/anime/:anime_id/episodes",
            get(get_anime_episodes_info_handler),
        )
        .route("/anime/:anime_id/staff", get(get_anime_staff_info_handler))
        .layer(Extension(client));

    Ok(router.into())
}
