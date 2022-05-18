use axum::http::StatusCode;
use crate::Error;

impl std::convert::From<crate::Error> for axum::http::StatusCode {
    fn from(err: Error) -> Self {
        match err {
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::InvalidArgument(_) => StatusCode::BAD_REQUEST,
            Error::GitError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::ParseOidError(_) => StatusCode::BAD_REQUEST,
            Error::SerializationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::MutexError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
