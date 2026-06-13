use std::{borrow::Borrow, cmp::Ordering};

pub trait OrdByKey {
    type Key: Ord;

    fn cmp(&self, other: &Self) -> Ordering;
    fn cmp_to_key<Q>(&self, key: &Q) -> Ordering
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized;
}
