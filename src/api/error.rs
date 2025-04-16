use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use std::fmt;

use crate::domain::RepositoryError;

/// API Error types that can be returned by handlers
#[derive(Debug)]
pub enum ApiError {
    /// Entity not found (404)
    NotFound(String),
    /// Bad request (400)
    BadRequest(String),
    /// Conflict (409)
    Conflict(String),
    /// Internal server error (500)
    Internal(String),
    /// Unauthorized (401)
    Unauthorized(String),
    /// Forbidden (403)
    Forbidden(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ApiError::NotFound(msg) => format!("Not found: {}", msg),
            ApiError::BadRequest(msg) => format!("Bad request: {}", msg),
            ApiError::Conflict(msg) => format!("Conflict: {}", msg),
            ApiError::Internal(msg) => format!("Internal error: {}", msg),
            ApiError::Unauthorized(msg) => format!("Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => format!("Forbidden: {}", msg),
        };
        write!(f, "{}", message)
    }
}

/// Transform ApiError into an HTTP response
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            ApiError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            ApiError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            ApiError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ApiError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        };

        // Create a JSON response body with error details
        let body = Json(json!({
            "error": {
                "type": status.canonical_reason(),
                "message": message,
            }
        }));

        // Combine the status code and body
        (status, body).into_response()
    }
}

/// Shorthand for API result type
pub type ApiResult<T> = Result<T, ApiError>;

/// Convert from domain RepositoryError to API error
impl From<RepositoryError> for ApiError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::NotFound(msg) => ApiError::NotFound(msg),
            RepositoryError::Conflict(msg) => ApiError::Conflict(msg),
            RepositoryError::Validation(msg) => ApiError::BadRequest(msg),
            RepositoryError::Internal(msg) => ApiError::Internal(msg),
        }
    }
}

/// Maps from RMCP errors to API errors
impl From<crate::mcp::methods::get_symbols::RmcpError> for ApiError {
    fn from(err: crate::mcp::methods::get_symbols::RmcpError) -> Self {
        use crate::mcp::methods::get_symbols::{RmcpError, RmcpErrorCode};

        match err {
            RmcpError::ParseError(msg) => ApiError::BadRequest(msg),
            RmcpError::RepositoryError(msg) => {
                // For repository errors, we can determine the type based on message content
                // since we don't have direct access to original RepositoryError
                if msg.starts_with("Not found:") {
                    ApiError::NotFound(msg)
                } else if msg.starts_with("Conflict:") {
                    ApiError::Conflict(msg)
                } else if msg.starts_with("Validation:") {
                    ApiError::BadRequest(msg)
                } else {
                    ApiError::Internal(msg)
                }
            }
            RmcpError::Other(msg) => ApiError::Internal(msg),
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
    fn test_repository_error_conversion() {
        let repo_error = RepositoryError::NotFound("Symbol not found".to_string());
        let api_error = ApiError::from(repo_error);

        match api_error {
            ApiError::NotFound(msg) => assert_eq!(msg, "Symbol not found"),
            _ => panic!("Conversion produced wrong error type"),
        }
    }
}
