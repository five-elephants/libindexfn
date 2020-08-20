use crate::{IdxResult,ObjectName,ObjectNameBuf,AccessStorage,Lookup};

use async_trait::async_trait;
use std::future::Future;

/// Trait to index storage systems.
///
/// This effectively creates a key -> object name mapping using a given keymap
/// function for all objects accessible through the AccessStorage trait starting
/// from a named directory.
#[async_trait]
pub trait Index<'a> {
    /// The type that is extracted from every object to create the index
    type Key: 'a;
    type Lookup: Lookup<'a, Key = Self::Key>;

    /// Constructor to perform indexing asynchronously
    async fn index<'b, S,F,U>(storage: &'b S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync,
            U: Future<Output = Self::Key> + Send,
            F: Fn(&'b S, ObjectNameBuf) -> U + Send + Sync;
}

