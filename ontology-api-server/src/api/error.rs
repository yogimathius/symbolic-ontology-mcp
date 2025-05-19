use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

use ontology_core::db::pool::DbError;
use ontology_core::db::repository::RepositoryError;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Conflict(String),
    Internal(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ApiError::NotFound(msg) => format!("Not found: {}", msg),
            ApiError::BadRequest(msg) => format!("Bad request: {}", msg),
            ApiError::Conflict(msg) => format!("Conflict: {}", msg),
            ApiError::Internal(msg) => format!("Internal error: {}", msg),
        };
        write!(f, "{}", message)
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "statusCode": status.as_u16(),
            "error": status.canonical_reason().unwrap_or("Unknown"),
            "message": message,
        }));

        (status, body).into_response()
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

impl From<DbError> for ApiError {
    fn from(err: DbError) -> Self {
        match err {
            DbError::NotFound => ApiError::NotFound("Record not found".to_string()),
            DbError::Conflict(msg) => ApiError::Conflict(msg),
            DbError::Connection(msg) => {
                ApiError::Internal(format!("Database connection error: {}", msg))
            }
            DbError::Sqlx(err) => ApiError::Internal(format!("Database error: {}", err)),
        }
    }
}

impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => ApiError::NotFound(msg),
            RepositoryError::Conflict(msg) => ApiError::Conflict(msg),
            RepositoryError::Validation(msg) => ApiError::BadRequest(msg),
            RepositoryError::Internal(msg) => ApiError::Internal(msg),
            RepositoryError::NotImplemented(msg) => {
                ApiError::Internal(format!("Not implemented: {}", msg))
            }
        }
    }
}

impl From<rmcp::Error> for ApiError {
    fn from(err: rmcp::Error) -> Self {
        let error_message = format!("{}", err);

        if error_message.contains("not found") || error_message.contains("NotFound") {
            ApiError::NotFound(error_message)
        } else if error_message.contains("invalid") || error_message.contains("Invalid") {
            ApiError::BadRequest(error_message)
        } else if error_message.contains("conflict") || error_message.contains("Conflict") {
            ApiError::Conflict(error_message)
        } else {
            ApiError::Internal(error_message)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_error_display() {
        let error = ApiError::NotFound("user".to_string());
        assert_eq!(error.to_string(), "Not found: user");
    }

    #[test]
    fn test_db_error_conversion() {
        let db_error = DbError::NotFound;
        let api_error = ApiError::from(db_error);

        match api_error {
            ApiError::NotFound(_) => { /* Test passed */ }
            _ => panic!("Conversion produced wrong error type"),
        }

        let db_error = DbError::Conflict("Duplicate key".to_string());
        let api_error = ApiError::from(db_error);

        match api_error {
            ApiError::Conflict(msg) => assert_eq!(msg, "Duplicate key"),
            _ => panic!("Conversion produced wrong error type"),
        }
    }
}
