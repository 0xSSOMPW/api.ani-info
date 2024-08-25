use crate::{
    db::{
        fetch_anime_detail_by_id, fetch_anime_ids, fetch_episodes_by_anime_id,
        fetch_staff_by_anime_id,
    },
    error::CustomErrorENUM,
    model::{Anime, AnimeID, AnimeStaff, Episode},
};
use axum::extract::Path;
use axum::{Extension, Json};
use std::sync::Arc;
use tokio_postgres::Client;

pub async fn get_anime_ids_handler(
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Vec<AnimeID>>, CustomErrorENUM> {
    let anime_ids = fetch_anime_ids(&client).await?;
    Ok(Json(anime_ids))
}

pub async fn get_anime_detail_by_id_handler(
    Path(anime_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Anime>, CustomErrorENUM> {
    let anime_details = fetch_anime_detail_by_id(&client, anime_id).await?;
    Ok(Json(anime_details))
}

pub async fn get_anime_episodes_info_handler(
    Path(anime_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Vec<Episode>>, CustomErrorENUM> {
    let episodes_info = fetch_episodes_by_anime_id(&client, anime_id).await?;
    Ok(Json(episodes_info))
}

pub async fn get_anime_staff_info_handler(
    Path(anime_id): Path<i32>,
    Extension(client): Extension<Arc<Client>>,
) -> Result<Json<Vec<AnimeStaff>>, CustomErrorENUM> {
    let staff_info = fetch_staff_by_anime_id(&client, anime_id).await?;
    Ok(Json(staff_info))
}
