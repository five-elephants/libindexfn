use super::{AccessStorage,ObjectName};
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

    fn make_path(&self, obj: ObjectName<'_>) -> PathBuf {
        let mut p = self.base_path.clone();
        p.push(Path::new(obj.name()));
        p
    }
}


#[async_trait]
impl AccessStorage for FileStorage {
    type ListIntoIter = Vec<String>;

    /// Read directory asynchronously using tokio, but not recursively
    async fn list(&self, dir_name: ObjectName<'_>) -> IdxResult<Self::ListIntoIter>
    {
        let path = self.make_path(dir_name);

        //println!("list({:?} in {:?})", dir_name.name(), self.base_path);
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


    async fn read_bytes(&self, obj_name: ObjectName<'_>) -> IdxResult<Vec<u8>> {
        let path = self.make_path(obj_name);

        let contents = fs::read(&path).await?;
        Ok(contents)
    }


    async fn write_bytes<T>(&self, name: ObjectName<'_>, data: T) -> IdxResult<()>
        where
            T: AsRef<[u8]> + Unpin + Send
    {
        let path = self.make_path(name);

        fs::write(&path, data).await?;
        Ok(())
    }
}
