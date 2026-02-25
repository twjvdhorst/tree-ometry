use std::borrow::Borrow;

pub trait Dynamic {
    type Key;
    type Value;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value>;

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(Self::Key, Self::Value)>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized;
    
    fn remove<Q>(&mut self, key: &Q) -> Option<Self::Value>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }
}
