
#[derive(Debug)]
pub enum IdxError {
    StorageError,
    JsonError(serde_json::error::Error)
}

pub type IdxResult<T> = Result<T, IdxError>;


impl From<serde_json::error::Error> for IdxError {
    fn from(err: serde_json::error::Error) -> Self {
        IdxError::JsonError(err)
    }
}


impl From<tokio::io::Error> for IdxError {
    fn from(_: tokio::io::Error) -> Self {
        IdxError::StorageError
    }
}
