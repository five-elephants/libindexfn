use super::{AccessStorage,is_valid_object_name};
use crate::error::*;

use std::path::{Path,PathBuf};
use async_trait::async_trait;
use tokio::fs;

pub struct FileStorage {
    base_path: PathBuf,
}


impl FileStorage {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self {
            base_path: path.into(),
        }
    }
}


#[async_trait]
impl AccessStorage for FileStorage {
    type ListIntoIter = Vec<String>;

    /// Read directory asynchronously using tokio, but not recursively
    async fn list<N>(&self, dir_name: N) -> IdxResult<Self::ListIntoIter>
        where
            N: AsRef<str> + Send
    {
        let path = {
            let mut p = self.base_path.clone();
            if !is_valid_object_name(dir_name.as_ref()) {
                return Err(IdxError::StorageError);
            }

            let dir_path = Path::new(dir_name.as_ref());
            p.push(dir_path);
            p
        };

        println!("list({:?} in {:?})", dir_name.as_ref(), self.base_path);
        let mut dir = fs::read_dir(path).await?;
        let mut rv = vec![];
        while let Some(entry) = dir.next_entry().await? {
            let entry_path = entry.path();
            let entry_rel_path = entry_path.strip_prefix(&self.base_path)
                .map_err(|_| IdxError::StorageError)?;

            let s = entry_rel_path.to_str()
                .ok_or(IdxError::StorageError)?
                .to_string();
            rv.push(s);
        }

        Ok(rv)
    }


    async fn read_bytes<N>(&self, obj_name: N) -> IdxResult<Vec<u8>>
        where
            N: AsRef<str> + Send
    {
        let path = {
            let mut p = self.base_path.clone();
            p.push(Path::new(obj_name.as_ref()));
            p
        };

        let contents = fs::read(&path).await?;
        Ok(contents)
    }


    async fn write_bytes<T,N>(&self, name: N, data: T) -> IdxResult<()>
        where
            T: AsRef<[u8]> + Unpin + Send,
            N: AsRef<str> + Send
    {
        let path = {
            let mut p = self.base_path.clone();
            p.push(Path::new(name.as_ref()));
            p
        };

        fs::write(&path, data).await?;
        Ok(())
    }
}
