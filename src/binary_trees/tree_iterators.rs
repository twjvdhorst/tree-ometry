use std::marker::PhantomData;

pub trait TreeIterator<T>: Iterator {
    fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
    where 
        P: FnMut(&T) -> bool;

    fn subtree_filter<P>(self, predicate: P) -> SubtreeFilter<Self, T, P>
    where 
        Self: Sized,
        P: FnMut(&T) -> bool,
    {
        SubtreeFilter { iter: self, predicate, _filter_arg: PhantomData }
    }
}

/// An iterator over the elements of a binary tree, where entire subtrees are filtered out if their roots don't pass the predicate.
/// Note that the api is different over the Filter<I, P> struct from Iterators.
/// The predicate is restricted to take only shared references as input, even if the iterator yields &mut items.
pub struct SubtreeFilter<I, T, P> {
    iter: I,
    predicate: P,
    _filter_arg: PhantomData<T>,
}

impl<I, T, P> Iterator for SubtreeFilter<I, T, P>
where 
    I: TreeIterator<T>,
    P: FnMut(&T) -> bool,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next_with_subtree_filter(&mut self.predicate)
    }
}
