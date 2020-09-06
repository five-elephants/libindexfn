use crate::{Lookup,IdxResult,IdxError,ObjectName};
use std::cmp;

#[derive(Debug)]
pub struct ScoredHit<T> {
    score: f64,
    item: T,
}

impl<T> ScoredHit<T> {
    pub fn new(score: f64, item: T) -> Self {
        Self {
            score,
            item
        }
    }

    pub fn score(&self) -> f64 {
        self.score
    }

    pub fn item(&self) -> &T {
        &self.item
    }
}


impl<T> PartialOrd for ScoredHit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.score.partial_cmp(&other.score)
    }
}

impl<T> PartialEq for ScoredHit<T> {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl<T> Eq for ScoredHit<T> { }


/// Find query in the index and return scored results
///
/// Higher scores signify a better match. Scores go from 0 to 1.
pub fn find_best_match<'a, L,F,Q,K>(
    lookup: &'a L,
    score: F,
    query: Q
) -> IdxResult<Vec<ScoredHit<ObjectName<'a>>>> 
    where
        L: Lookup<'a>,
        F: Fn(&Q, K) -> f64,
        <L as Lookup<'a>>::Key: std::fmt::Debug,
        K: From<&'a <L as Lookup<'a>>::Key>,
        Q: std::fmt::Debug
{
    let mut rv = Vec::new();

    // compute score for all keys
    for key in lookup.keys() {
        let k: K = key.into();
        let s = score(&query, k);
        if s.is_nan() {
            let msg = format!("Score evaluates to NaN for key '{:?}' and query '{:?}'",
                key, query);
            return Err(IdxError::indexing_error_msg(msg));
        }
        let objs = lookup.get(key)?;

        for obj in objs {
            rv.push(ScoredHit::new(s, obj));
        }
    }

    // sort by score, descending
    rv.sort_unstable_by(|a,b| a.partial_cmp(b).unwrap());
    rv.reverse();

    Ok(rv)
}

