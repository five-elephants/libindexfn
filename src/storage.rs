pub(crate) mod fs;

use crate::IdxResult;

use async_trait::async_trait;
use serde::{Serialize,de::DeserializeOwned};

#[async_trait]
pub trait AccessStorage {
    type ListIntoIter: IntoIterator<Item = String>;

    /// List directory contents
    async fn list<N>(&self, dir_name: N) -> IdxResult<Self::ListIntoIter>
        where
            N: AsRef<str> + Send;

    /// Read raw bytes from an object
    async fn read_bytes<N>(&self, obj_name: N) -> IdxResult<Vec<u8>>
        where
            N: AsRef<str> + Send;

    /// Write an object by providing raw bytes
    async fn write_bytes<T,N>(&self, name: N, data: T) -> IdxResult<()>
        where
            T: AsRef<[u8]> + Unpin + Send,
            N: AsRef<str> + Send;

    /// Read a JSON object and directly deserialize it before returning
    async fn read_json<T, N>(&self, obj_name: N) -> IdxResult<Box<T>>
        where
            T: DeserializeOwned,
            N: AsRef<str> + Send
    {
        let byte_data = self.read_bytes(obj_name).await?;

        Ok(serde_json::from_slice(&byte_data)?)
    }

    /// Write an object as JSON file
    async fn write_json<T, N, M>(&self, obj_name: N, obj: M) -> IdxResult<()>
        where
            T: Serialize,
            N: AsRef<str> + Send,
            M: AsRef<T> + Send
    {
        let byte_data = serde_json::to_vec(obj.as_ref())?;

        self.write_bytes(obj_name, byte_data).await
    }
}


pub(crate) fn is_valid_object_name(name: &str) -> bool {
    name.chars()
        .all(|c| {
            char::is_alphanumeric(c)
                || (c == '_')
                || (c == '-')

        })
}

