use crate::{IdxError,IdxResult,ObjectName,ObjectNameBuf,AccessStorage,Lookup};

use async_trait::async_trait;
use std::future::Future;

/// Trait to index storage systems.
///
/// This effectively creates a key -> object name mapping using a given keymap
/// function for all objects accessible through the AccessStorage trait starting
/// from a named directory.
#[async_trait]
pub trait Index<'a> 
    where
        IdxError: From<Self::Error>
{
    /// The type that is extracted from every object to create the index
    type Key: 'a;
    type Lookup: Lookup<'a, Key = Self::Key>;
    type Error: Send;

    /// Constructor to perform indexing asynchronously
    async fn index<S,F,U>(storage: &S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync + Send + Clone + 'static,
            U: Future<Output = Result<Self::Key, Self::Error>> + Send,
            F: Fn(S, ObjectNameBuf) -> U + Send + Sync + Clone + 'static;
}


/// Trait to index storage like Index, but produce multiple keys for each object
#[async_trait]
pub trait MultiIndex<'a>
    where
        IdxError: From<Self::Error>
{
    /// The type that is extracted from every object to create the index
    type Key: 'a;
    type Lookup: Lookup<'a, Key = Self::Key>;
    type Error: Send;

    /// Constructor to perform indexing asynchronously
    async fn multi_index<S,F,U>(storage: &S, start: ObjectName<'_>, keymap: F)
            -> IdxResult<Self::Lookup>
        where
            S: AccessStorage + Sync + Send + Clone + 'static,
            U: Future<Output = Result<Vec<Self::Key>, Self::Error>> + Send,
            F: Fn(S, ObjectNameBuf) -> U + Send + Sync + Clone + 'static;
}


//#[async_trait]
//pub trait ScoredMultiIndex<'a>
    //where
        //IdxError: From<Self::Error>
//{
    //type Key: 'a;
    //type Query;
    //type Lookup: ScoredLookup<'a, Query = Self::Query>;
    //type Error: Send;

    //async fn scored_multi_index<S,F,U>(
        //storage: &S,
        //start: ObjectName<'_>, 
        //extractor: F
    //) -> IdxResult<Self::Lookup>
        //where
            //S: AccessStorage + Sync + Send + Clone + 'static,
            //U: Future<Output = Result<Vec<Self::Key>, Self::Error>> + Send,
            //F: Fn(S, ObjectNameBuf) -> U + Send + Sync + Clone + 'static;
//}

