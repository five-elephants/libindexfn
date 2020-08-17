use crate::{IdxResult,ObjectName,AccessStorage,Lookup};

use async_trait::async_trait;

/// Trait to index storage systems.
///
/// This effectively creates a key -> object name mapping using a given keymap
/// function for all objects accessible through the AccessStorage trait starting
/// from a named directory.
#[async_trait]
pub trait Index<'a> {
    /// The type that is extracted from every object to create the index
    type Key;
    type Lookup: Lookup<'a, Key = Self::Key>;

    /// Constructor to perform indexing asynchronously
    async fn index<S,F>(storage: &S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync,
            F: Fn(&S, ObjectName<'_>) -> Self::Key + Send;
}

