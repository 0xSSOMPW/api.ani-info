#[derive(Debug)]
pub enum CustomErrorENUM {
    DatabaseError(tokio_postgres::Error),
    NotFound,
}

impl std::fmt::Display for CustomErrorENUM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CustomErrorENUM::DatabaseError(err) => write!(f, "Database error: {}", err),
            CustomErrorENUM::NotFound => write!(f, "Anime not found"),
        }
    }
}
