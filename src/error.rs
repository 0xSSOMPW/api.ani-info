use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::{fmt, sync::Arc};

#[derive(Debug)]
pub enum CustomErrorENUM {
    DatabaseError(tokio_postgres::Error),
    DatabaseRefError(String),
    NotFound,
    TlsError(native_tls::Error),
    TlsRefError(String),
}

// Implement Display for CustomErrorENUM
impl fmt::Display for CustomErrorENUM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CustomErrorENUM::DatabaseError(err) => write!(f, "Database error: {}", err),
            CustomErrorENUM::DatabaseRefError(err) => write!(f, "Database error: {}", err),
            CustomErrorENUM::NotFound => write!(f, "Anime not found"),
            CustomErrorENUM::TlsError(err) => write!(f, "TLS error: {}", err),
            CustomErrorENUM::TlsRefError(err) => write!(f, "TLS error: {}", err),
        }
    }
}

// Implement From for Arc<CustomErrorENUM>
impl From<Arc<CustomErrorENUM>> for CustomErrorENUM {
    fn from(arc: Arc<CustomErrorENUM>) -> Self {
        // You can choose to clone the error or match it to create the appropriate variant
        match &*arc {
            CustomErrorENUM::NotFound => CustomErrorENUM::NotFound,
            CustomErrorENUM::DatabaseError(e) => CustomErrorENUM::DatabaseRefError(e.to_string()),
            CustomErrorENUM::TlsError(e) => CustomErrorENUM::TlsRefError(e.to_string()),
            CustomErrorENUM::DatabaseRefError(e) => {
                CustomErrorENUM::DatabaseRefError(e.to_string())
            }
            CustomErrorENUM::TlsRefError(e) => CustomErrorENUM::TlsRefError(e.to_string()),
        }
    }
}

// Implement IntoResponse for CustomErrorENUM
impl IntoResponse for CustomErrorENUM {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            CustomErrorENUM::DatabaseRefError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            CustomErrorENUM::DatabaseError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", err),
            ),
            CustomErrorENUM::NotFound => (StatusCode::NOT_FOUND, "Anime not found".to_string()),
            CustomErrorENUM::TlsRefError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("TLS error: {}", err),
            ),
            CustomErrorENUM::TlsError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("TLS error: {}", err),
            ),
        };
        (status, body).into_response()
    }
}
