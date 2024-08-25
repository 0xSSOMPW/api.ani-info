use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use native_tls;
use std::fmt;

#[derive(Debug)]
pub enum CustomErrorENUM {
    DatabaseError(tokio_postgres::Error),
    NotFound,
    IoError(std::io::Error),
    TlsError(native_tls::Error),
    Base64DecodeError(base64::DecodeError),
}

// Implement Display for CustomErrorENUM
impl fmt::Display for CustomErrorENUM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomErrorENUM::DatabaseError(err) => write!(f, "Database error: {}", err),
            CustomErrorENUM::NotFound => write!(f, "Anime not found"),
            CustomErrorENUM::IoError(err) => write!(f, "IO error: {}", err),
            CustomErrorENUM::TlsError(err) => write!(f, "TLS error: {}", err),
            CustomErrorENUM::Base64DecodeError(err) => write!(f, "Base64 decode error: {}", err),
        }
    }
}

// Implement IntoResponse for CustomErrorENUM
impl IntoResponse for CustomErrorENUM {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            CustomErrorENUM::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            CustomErrorENUM::NotFound => (StatusCode::NOT_FOUND, "Anime not found".to_string()),
            CustomErrorENUM::IoError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("IO error: {}", err),
            ),
            CustomErrorENUM::TlsError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("TLS error: {}", err),
            ),
            CustomErrorENUM::Base64DecodeError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Base64 decode error: {}", err),
            ),
        };
        (status, body).into_response()
    }
}
