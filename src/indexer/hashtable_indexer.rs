use crate::{IdxResult,ObjectName,ObjectNameBuf,Lookup,Index,AccessStorage};

use async_trait::async_trait;
use std::collections::{hash_map,HashMap};
use std::hash::Hash;
use std::future::Future;

pub struct HashTableIndexer<K> {
    map: HashMap<K,Vec<ObjectNameBuf>>
}


#[async_trait]
impl<'a, K: 'a + Eq + Hash + Send> Index<'a> for HashTableIndexer<K> {
    type Key = K;
    type Lookup = Self;

    async fn index<'b, S,F,U>(storage: &'b S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync,
            U: Future<Output = Self::Key> + Send,
            F: Fn(&'b S, ObjectNameBuf) -> U + Send + Sync
    {
        let mut map = HashMap::new();
        let listing: Vec<_> = storage.list(start).await?.into_iter().collect();

        for file in listing {
            let filename = ObjectNameBuf::from_str(&file)?;
            let key = keymap(storage, filename.clone()).await;

            map.entry(key).or_insert(vec![]).push(filename);
        }

        let rv = Self {
            map: map
        };

        Ok(rv)
    }
}


impl<'a, K: 'a + Eq + Hash> Lookup<'a> for HashTableIndexer<K> {
    type Key = K;
    type KeyIter = hash_map::Keys<'a, Self::Key, Vec<ObjectNameBuf>>;

    fn get(&'a self, key: &Self::Key) -> IdxResult<Vec<ObjectName<'a>>> {
        if let Some(res) = self.map.get(key) {
            let mut rv = Vec::with_capacity(res.len());

            for entry in res.iter() {
                rv.push(entry.name());
            }

            Ok(rv)
        } else {
            Ok(vec![])
        }
    }


    fn keys(&'a self) -> Self::KeyIter {
        self.map.keys()
    }
}
