use serde::Serialize;
use tracing::error;

#[derive(Debug, Serialize)]
pub enum ErrorType {
    ClientError,
    ServerError,
}

#[derive(Debug, Serialize)]
pub struct AppError {
    pub error_type: ErrorType,
    pub detail: String,
}

impl AppError {
    pub fn client_error(detail: String) -> Self {

        Self {
            error_type: ErrorType::ClientError,
            detail: detail,
        }
    }

    pub fn server_error(details: impl Into<String>) -> Self {
        let details = details.into();

        error!("Internal error: {}", details);

        Self {
            error_type: ErrorType::ServerError,
            detail: "Internal error".to_string(),
        }
    }
}