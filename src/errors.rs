use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum StateError {
    Unauthorized,
    NotFound,
    UnableToParse,
    AlreadyTaken,
    ServerError,
    UnsupportedDataType(String),
}

impl IntoResponse for StateError {
    fn into_response(self) -> axum::response::Response {
        let mut status_code = None;
        let mut body: Option<Json<Self>> = None;
        match self {
            Self::Unauthorized => {
                status_code.replace(StatusCode::FORBIDDEN);
            }
            Self::NotFound => {
                status_code.replace(StatusCode::NOT_FOUND);
            }
            Self::UnableToParse => {
                body.replace(self.into());
            }
            Self::AlreadyTaken => {
                body.replace(self.into());
            }
            Self::UnsupportedDataType(_) => {
                body.replace(self.into());
            }
            Self::ServerError => {
                status_code.replace(StatusCode::INTERNAL_SERVER_ERROR);
            }
        };
        match body {
            Some(data) => (status_code.unwrap_or(StatusCode::OK), data).into_response(),
            None => status_code.unwrap_or(StatusCode::OK).into_response(),
        }
    }
}
