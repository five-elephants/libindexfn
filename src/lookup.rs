use crate::{IdxResult,ObjectName};

/// Retrieving object names from an index
pub trait Lookup<'a> {
    /// Type used as key into index
    type Key: 'a;

    /// Iterator type for keys
    type KeyIter: Iterator<Item = &'a Self::Key>;

    /// Return all object names belonging to the given key
    fn get(&'a self, key: &Self::Key) -> IdxResult<Vec<ObjectName<'a>>>;

    /// Iterate over keys in index in arbitrary order
    fn keys(&'a self) -> Self::KeyIter;
}
