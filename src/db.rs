use crate::error::CustomErrorENUM;
use crate::model::{Anime, AnimeID, AnimeStaff, Episode};
use crate::query::Query;
use base64::decode;
use native_tls::{Certificate, TlsConnector};
use postgres_native_tls::MakeTlsConnector;
use tokio_postgres::Client;

pub async fn establish_connection(
    database_url: &str,
    ca_cert: &str,
) -> Result<Client, CustomErrorENUM> {
    let ca_cert_bytes = decode(ca_cert).map_err(CustomErrorENUM::Base64DecodeError)?;
    let cert = Certificate::from_pem(&ca_cert_bytes).map_err(CustomErrorENUM::TlsError)?;

    let connector = TlsConnector::builder()
        .add_root_certificate(cert)
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(CustomErrorENUM::TlsError)?;

    let make_tls_connector = MakeTlsConnector::new(connector);

    let (client, connection) = tokio_postgres::connect(database_url, make_tls_connector)
        .await
        .map_err(CustomErrorENUM::DatabaseError)?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {:?}", e);
        }
    });

    Ok(client)
}

pub async fn fetch_anime_ids(client: &Client) -> Result<Vec<AnimeID>, CustomErrorENUM> {
    let rows = client
        .query(Query::AnimeId.sql(), &[])
        .await
        .map_err(CustomErrorENUM::DatabaseError)?;

    let anime_ids: Vec<AnimeID> = rows
        .iter()
        .map(|row| AnimeID {
            id: row.get("id"),
            anime_name: row.get("anime_name"),
        })
        .collect();

    Ok(anime_ids)
}

pub async fn fetch_anime_detail_by_id(
    client: &Client,
    anime_id: i32,
) -> Result<Anime, CustomErrorENUM> {
    let rows = client
        .query(Query::AnimeById.sql(), &[&(anime_id)])
        .await
        .map_err(CustomErrorENUM::DatabaseError)?;

    if rows.is_empty() {
        Err(CustomErrorENUM::NotFound)
    } else {
        let row = &rows[0];
        let anime = Anime {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            mal_id: row.get("mal_id"),
            al_id: row.get("al_id"),
            japanese_title: row.get("japanese_title"),
            synonyms: row.get("synonyms"),
            image: row.get("image"),
            category: row.get("category"),
            rating: row.get("rating"),
            quality: row.get("quality"),
            duration: row.get("duration"),
            premiered: row.get("premiered"),
            aired: row.get("aired"),
            status: row.get("status"),
            mal_score: row.get("mal_score"),
            studios: row.get("studios"),
            producers: row.get("producers"),
            genres: row.get("genres"),
            sub_episodes: row.get("sub_episodes"),
            dub_episodes: row.get("dub_episodes"),
            total_episodes: row.get("total_episodes"),
            sub_or_dub: row.get("sub_or_dub"),
        };
        Ok(anime)
    }
}

pub async fn fetch_episodes_by_anime_id(
    client: &Client,
    anime_id: i32,
) -> Result<Vec<Episode>, CustomErrorENUM> {
    let rows = client
        .query(Query::EpisodesByAnimeId.sql(), &[&(anime_id)])
        .await
        .map_err(CustomErrorENUM::DatabaseError)?;

    if rows.is_empty() {
        Err(CustomErrorENUM::NotFound)
    } else {
        let episodes: Vec<Episode> = rows
            .iter()
            .map(|row| Episode {
                id: row.get("id"),
                title: row.get("title"),
                episode_no: row.get("episode_no"),
                is_filler: row.get("is_filler"),
                anime_id: row.get("anime_id"),
            })
            .collect();
        Ok(episodes)
    }
}

pub async fn fetch_staff_by_anime_id(
    client: &Client,
    anime_id: i32,
) -> Result<Vec<AnimeStaff>, CustomErrorENUM> {
    let rows = client
        .query(Query::StaffByAnimeId.sql(), &[&(anime_id)])
        .await
        .map_err(CustomErrorENUM::DatabaseError)?;

    if rows.is_empty() {
        Err(CustomErrorENUM::NotFound)
    } else {
        let episodes: Vec<AnimeStaff> = rows
            .iter()
            .map(|row| AnimeStaff {
                mal_id: row.get("mal_id"),
                name: row.get("name"),
                image: row.get("image"),
                mal_url: row.get("mal_url"),
                positions: row.get("positions"),
            })
            .collect();
        Ok(episodes)
    }
}
