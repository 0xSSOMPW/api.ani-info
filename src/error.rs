use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum CustomErrorENUM {
    DatabaseError(tokio_postgres::Error),
    NotFound,
}

// Implement Display for CustomErrorENUM
impl fmt::Display for CustomErrorENUM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomErrorENUM::DatabaseError(err) => write!(f, "Database error: {}", err),
            CustomErrorENUM::NotFound => write!(f, "Anime not found"),
        }
    }
}

// Implement IntoResponse for CustomErrorENUM
impl IntoResponse for CustomErrorENUM {
    fn into_response(self) -> Response {
        match self {
            CustomErrorENUM::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            )
                .into_response(),
            CustomErrorENUM::NotFound => {
                (StatusCode::NOT_FOUND, "Anime not found".to_string()).into_response()
            }
        }
    }
}
