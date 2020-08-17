pub(crate) mod fs;

use crate::{IdxResult,IdxError};

use async_trait::async_trait;
use serde::{Serialize,de::DeserializeOwned};


#[derive(Clone,Copy)]
pub struct ObjectName<'a> {
    name: &'a str
}

impl<'a> ObjectName<'a> {
    pub fn new(name: &'a str) -> IdxResult<Self> {
        if Self::is_valid_object_name(name) {
            Ok(Self {
                name: name
            })
        } else {
            Err(IdxError::StorageError)
        }
    }

    pub fn empty() -> Self {
        Self {
            name: ""
        }
    }

    fn is_valid_object_name(name: &str) -> bool {
        name.chars()
            .all(|c| {
                char::is_alphanumeric(c)
                    || (c == '_')
                    || (c == '-')
                    || (c == '.')

            })
    }

    pub fn name(&'a self) -> &'a str {
        self.name
    }
}


#[async_trait]
pub trait AccessStorage {
    type ListIntoIter: IntoIterator<Item = String>;

    /// List directory contents
    async fn list(&self, dir_name: ObjectName<'_>) -> IdxResult<Self::ListIntoIter>;

    /// Read raw bytes from an object
    async fn read_bytes(&self, obj_name: ObjectName<'_>) -> IdxResult<Vec<u8>>;

    /// Write an object by providing raw bytes
    async fn write_bytes<T>(&self, name: ObjectName<'_>, data: T) -> IdxResult<()>
        where
            T: AsRef<[u8]> + Unpin + Send;

    /// Read a JSON object and directly deserialize it before returning
    async fn read_json<T>(&self, obj_name: ObjectName<'_>) -> IdxResult<Box<T>>
        where
            T: DeserializeOwned
    {
        let byte_data = self.read_bytes(obj_name).await?;

        Ok(serde_json::from_slice(&byte_data)?)
    }

    /// Write an object as JSON file
    async fn write_json<T>(&self, obj_name: ObjectName<'_>, obj: T) -> IdxResult<()>
        where
            T: Serialize + Send
    {
        let byte_data = serde_json::to_vec(&obj)?;

        self.write_bytes(obj_name, byte_data).await
    }
}

