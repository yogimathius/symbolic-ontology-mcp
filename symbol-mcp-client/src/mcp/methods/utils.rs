use crate::mcp::methods::get_symbols::RmcpError;
use ontology_core::db::repository::RepositoryError;

/// Helper function to convert RepositoryError to RmcpError
/// Can be reused across all MCP method handlers
pub fn repository_error_to_rmcp_error(err: RepositoryError) -> RmcpError {
    match err {
        RepositoryError::NotFound(msg) => RmcpError::RepositoryError(format!("Not found: {}", msg)),
        RepositoryError::Conflict(msg) => RmcpError::RepositoryError(format!("Conflict: {}", msg)),
        RepositoryError::Internal(msg) => {
            RmcpError::RepositoryError(format!("Internal error: {}", msg))
        }
        RepositoryError::Validation(msg) => {
            RmcpError::RepositoryError(format!("Validation error: {}", msg))
        }
        RepositoryError::NotImplemented(msg) => {
            RmcpError::RepositoryError(format!("Not implemented: {}", msg))
        }
    }
}
