use crate::{
    db::{
        fetch_anime_detail_by_al_id, fetch_anime_detail_by_id, fetch_anime_detail_by_mal_id,
        fetch_anime_ids, fetch_episodes_by_anime_id, fetch_staff_by_anime_id,
    },
    error::CustomErrorENUM,
    model::{Anime, AnimeID, AnimeStaff, Episode},
};
use axum::extract::Path;
use axum::{Extension, Json};
use moka::future::Cache;
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;
use tokio_postgres::Client;

pub struct HiAnime {
    anime_ids_list_cache: Cache<String, Vec<AnimeID>>,
    anime_details_cache: Cache<i32, Anime>,
    anime_mal_ids_details_cache: Cache<i32, Anime>,
    anime_al_ids_details_cache: Cache<i32, Anime>,
    anime_episodes_list_cache: Cache<i32, Vec<Episode>>,
    anime_staffs_list_cache: Cache<i32, Vec<AnimeStaff>>,
}

impl HiAnime {
    pub fn new() -> Self {
        Self {
            anime_ids_list_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            anime_details_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            anime_mal_ids_details_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            anime_al_ids_details_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            anime_episodes_list_cache: Cache::builder()
                .time_to_live(Duration::from_secs(7200)) // 2 hours
                .build(),
            anime_staffs_list_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
        }
    }

    pub async fn get_anime_ids_handler(
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Vec<AnimeID>>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let anime_ids = cache
            .anime_ids_list_cache
            .try_get_with("anime_ids".to_string(), async {
                fetch_anime_ids(&client)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(anime_ids))
    }

    pub async fn get_anime_detail_by_id_handler(
        Path(anime_id): Path<i32>,
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Anime>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let anime_details = cache
            .anime_details_cache
            .try_get_with(anime_id, async {
                fetch_anime_detail_by_id(&client, anime_id)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(anime_details))
    }

    pub async fn get_anime_detail_by_mal_id_handler(
        Path(anime_mal_id): Path<i32>,
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Anime>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let anime_details = cache
            .anime_mal_ids_details_cache
            .try_get_with(anime_mal_id, async {
                fetch_anime_detail_by_mal_id(&client, anime_mal_id)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(anime_details))
    }

    pub async fn get_anime_detail_by_al_id_handler(
        Path(anime_al_id): Path<i32>,
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Anime>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let anime_details = cache
            .anime_mal_ids_details_cache
            .try_get_with(anime_al_id, async {
                fetch_anime_detail_by_al_id(&client, anime_al_id)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(anime_details))
    }

    pub async fn get_anime_episodes_info_handler(
        Path(anime_id): Path<i32>,
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Vec<Episode>>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let episodes_info = cache
            .anime_episodes_list_cache
            .try_get_with(anime_id, async {
                fetch_episodes_by_anime_id(&client, anime_id)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(episodes_info))
    }

    pub async fn get_anime_staff_info_handler(
        Path(anime_id): Path<i32>,
        Extension(client): Extension<Arc<Client>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<Vec<AnimeStaff>>, CustomErrorENUM> {
        let cache = cache.lock().await;
        let staff_info = cache
            .anime_staffs_list_cache
            .try_get_with(anime_id, async {
                fetch_staff_by_anime_id(&client, anime_id)
                    .await
                    .map_err(CustomErrorENUM::from)
            })
            .await?;
        Ok(Json(staff_info))
    }
}
