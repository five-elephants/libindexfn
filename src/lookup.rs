use crate::{IdxResult,ObjectName};

/// Retrieving object names from an index
pub trait Lookup<'a> {
    /// Type used as key into index
    type Key;

    /// Return all object names belonging to the given key
    fn get(&'a self, key: &Self::Key) -> IdxResult<Vec<ObjectName<'a>>>;

    /// Return an implementor of Lookup that contains just the objects, for which
    /// flter(key) evaluates to true.
    fn filter<F,T>(&self, fltr: F) -> T
        where
            F: Fn(&Self::Key) -> bool,
            T: Lookup<'a>;
}
