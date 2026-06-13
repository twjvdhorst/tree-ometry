use std::{borrow::Borrow, cmp::Ordering};

pub trait OrdByKey {
    type Key: Ord;

    fn key(&self) -> &Self::Key;

    fn cmp(&self, other: &Self) -> Ordering {
        <Self::Key as Ord>::cmp(self.key(), other.key())
    }

    fn cmp_to_key<Q>(&self, key: &Q) -> Ordering
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.key().borrow().cmp(key)
    }
}
