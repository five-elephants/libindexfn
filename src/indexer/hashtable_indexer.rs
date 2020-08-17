use crate::{IdxResult,ObjectName,Lookup,Index,AccessStorage};

use async_trait::async_trait;
use std::collections::HashMap;
use std::hash::Hash;

pub struct HashTableIndexer<K> {
    map: HashMap<K,Vec<String>>
}


#[async_trait]
impl<'a, K: Eq + Hash> Index<'a> for HashTableIndexer<K> {
    type Key = K;
    type Lookup = Self;

    async fn index<S,F>(storage: &S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync,
            F: Fn(&S, ObjectName<'_>) -> Self::Key + Send
    {
        unimplemented!()
    }
}


impl<'a, K: Eq + Hash> Lookup<'a> for HashTableIndexer<K> {
    type Key = K;

    fn get(&'a self, key: &Self::Key) -> IdxResult<Vec<ObjectName<'a>>> {
        if let Some(res) = self.map.get(key) {
            let mut rv = Vec::with_capacity(res.len());

            for entry in res.iter() {
                let name = ObjectName::new(entry)?;
                rv.push(name);
            }

            Ok(rv)
        } else {
            Ok(vec![])
        }
    }

    fn filter<F,T>(&self, fltr: F) -> T
        where
            F: Fn(&Self::Key) -> bool,
            T: Lookup<'a>
    {
        unimplemented!()
    }
}
