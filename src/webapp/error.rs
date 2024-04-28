use axum::response::IntoResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Not Found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Invalid input")]
    InvalidInput,
}

impl HttpError {
    pub fn status_code(&self) -> u16 {
        match self {
            HttpError::NotFound => 404,
            HttpError::InternalServerError => 500,
            HttpError::InvalidInput => 400,
        }
    }
}

impl IntoResponse for HttpError {
    fn into_response(self) -> axum::response::Response {
        // error!("error happened: {}", self);
        match self {
            HttpError::NotFound => axum::http::StatusCode::NOT_FOUND.into_response(),
            HttpError::InternalServerError => {
                axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            HttpError::InvalidInput => axum::http::StatusCode::BAD_REQUEST.into_response(),
        }
    }
}
