// Queries as constants
const ANIME_ID_QUERY: &str = "
    SELECT 
        id, 
        anime_name 
    FROM 
        anime_id
";

const ANIME_FIND_BY_ID_QUERY: &str = "
    SELECT 
        id, 
        title, 
        description, 
        mal_id, 
        al_id, 
        japanese_title, 
        synonyms, 
        image, 
        category, 
        rating, 
        quality, 
        duration, 
        premiered, 
        aired, 
        status, 
        mal_score, 
        studios, 
        producers, 
        genres, 
        sub_episodes, 
        dub_episodes, 
        total_episodes, 
        sub_or_dub 
    FROM 
        anime 
    WHERE 
        id = $1
";

const EPISODES_QUERY: &str = "
    SELECT 
        id, 
        title,
        episode_no, 
        is_filler, 
        anime_id 
    FROM 
        episodes 
    WHERE 
        anime_id = $1
";

// Enum to represent queries
#[derive(Debug)]
pub enum Query {
    AnimeId,
    AnimeById,
    EpisodesByAnimeId,
}

// Implementing a method to get the SQL query from the enum
impl Query {
    pub fn sql(&self) -> &str {
        match self {
            Query::AnimeId => ANIME_ID_QUERY,
            Query::AnimeById => ANIME_FIND_BY_ID_QUERY,
            Query::EpisodesByAnimeId => EPISODES_QUERY,
        }
    }
}
