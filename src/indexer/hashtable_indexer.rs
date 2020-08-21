use crate::{IdxResult,ObjectName,ObjectNameBuf,Lookup,Index,AccessStorage};

use tokio::spawn;
use tokio::sync::mpsc;
use async_trait::async_trait;
use std::collections::{hash_map,HashMap};
use std::hash::Hash;
use std::future::Future;


pub struct HashTableIndexer<K> {
    map: HashMap<K,Vec<ObjectNameBuf>>
}


#[async_trait]
impl<'a, K: 'static + Eq + Hash + Send> Index<'a> for HashTableIndexer<K> {
    type Key = K;
    type Lookup = Self;

    async fn index<S,F,U>(storage: &S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Clone + Send + Sync + 'static,
            U: Future<Output = Self::Key> + Send,
            F: Fn(S, ObjectNameBuf) -> U + Send + Sync + Clone + 'static
    {
        // Set up a channel to return computed keys from indexing tasks
        let (tx, mut rx) = mpsc::channel(100);
        // Vec for task JoinHandles
        let mut index_tasks = Vec::new();

        // List files in storage and start a task for each one
        for file in storage.list(start).await?.into_iter() {
            let f = ObjectNameBuf::from_str(&file)?;

            let handle = {
                // clone everything to pass to the async block inside the task
                // data in the task has to have 'static lifetime
                let mut tx = tx.clone();
                let storage = storage.clone();
                let keymap: F = keymap.clone();

                spawn(async move {
                    let key = keymap(storage, f.clone()).await;
                    if let Err(_) = tx.send((key, f)).await {
                        panic!("Unexpected error: receiver dropped");
                    }
                })
            };
            index_tasks.push(handle);
        }

        // need to drop receiver threads/tasks Sender, so that while loop below can terminate 
        drop(tx);

        // collect results from channel into index HashMap
        let mut map = HashMap::new();
        while let Some((key, filename)) = rx.recv().await {
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
