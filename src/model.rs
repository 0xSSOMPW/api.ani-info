use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Anime {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub mal_id: i32,
    pub al_id: i32,
    pub japanese_title: Option<String>,
    pub synonyms: Option<String>,
    pub image: String,
    pub category: String,
    pub rating: String,
    pub quality: String,
    pub duration: String,
    pub premiered: String,
    pub aired: String,
    pub status: String,
    pub mal_score: String,
    pub studios: String,
    pub producers: String,
    pub genres: String,
    pub sub_episodes: i32,
    pub dub_episodes: i32,
    pub total_episodes: i32,
    pub sub_or_dub: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnimeID {
    pub id: i32,
    pub anime_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Episode {
    pub id: String,
    pub title: String,
    pub is_filler: bool,
    pub episode_no: i32,
    pub anime_id: i32,
}
