use serde::Serialize;

#[derive(Debug, Serialize, Clone)]
pub enum StateError {
    Unauthorized,
    NotFound,
    AlreadyTaken,
    ServerError,
    UnsupportedDataType(String),
}
