use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum IdxError {
    StorageError(Box<dyn Error>),
    JsonError(serde_json::error::Error)
}

impl IdxError {
    pub fn storage_error<T: Error + 'static>(e: T) -> Self {
        Self::StorageError(Box::new(e))
    }

    pub fn storage_error_msg(s: impl Into<String>) -> Self {
        let msg = Message(s.into());
        Self::storage_error(msg)
    }
}

impl fmt::Display for IdxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StorageError(err) => {
                write!(f, "Storage error: {}", err)
            }

            Self::JsonError(err) => {
                write!(f, "JSON error: {}", err)
            }
        }
    }
}

impl Error for IdxError { }

pub type IdxResult<T> = Result<T, IdxError>;


impl From<serde_json::error::Error> for IdxError {
    fn from(err: serde_json::error::Error) -> Self {
        IdxError::JsonError(err)
    }
}


impl From<tokio::io::Error> for IdxError {
    fn from(e: tokio::io::Error) -> Self {
        IdxError::StorageError(Box::new(e))
    }
}



#[derive(Debug)]
pub struct Message(pub String);

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for Message { }