use std::borrow::Borrow;

pub trait Dynamic {
    type Key;
    type Value;

    /// Inserts the key-value pair into the tree.
    /// If the key was not present in the tree yet, None is returned.
    /// Otherwise, the value stored at the given key is updated, and the old value is returned.
    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    /// Removes the node with the given key from the tree.
    /// Returns the key and associated value.
    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(Self::Key, Self::Value)>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized;
    
    /// Removes the node with the given key from the tree.
    /// Returns the associated value.
    fn remove<Q>(&mut self, key: &Q) -> Option<Self::Value>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }
}
