use axum::http::StatusCode;

pub(crate) trait ToErrorResponse {
    fn to_response(&self) -> (StatusCode, String);
}

impl ToErrorResponse for service::error::ServiceError {
    fn to_response(&self) -> (StatusCode, String) {
        match self {
            service::error::ServiceError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, self.to_string())
            }
            service::error::ServiceError::FileNotFound(_) => {
                (StatusCode::NOT_FOUND, self.to_string())
            }
            service::error::ServiceError::CouldNotDeleteFile(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
        }
    }
}
