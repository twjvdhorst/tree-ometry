use std::borrow::Borrow;

use super::{
    Cursor,
    CursorMut,
    TreeSemigroup,
};
use crate::binary_trees::{
    binary_search_trees::{
        red_black_trees::{
            base, 
            ord_by_key::OrdByKey,
        },
        semigroup_rb_tree::iterators::{
            InorderIter,
            InorderIterMut,
            IntoInorderIter,
        },
    },
    binary_tree_cursor::PeekingCursorMut,
};

/// Struct containing the data in each node of the tree.
/// Values are considered equal if their keys are equal, regardless of what other data they store.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SemigroupRbData<K, V, S> {
    key: K,
    value: V,
    semigroup_value: S,
}

impl<K, V, S> OrdByKey for SemigroupRbData<K, V, S>
where 
    K: Ord,
{
    type Key = K;

    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.key.cmp(&other.key)
    }

    fn cmp_to_key<Q>(&self, key: &Q) -> std::cmp::Ordering
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized
    {
        self.key.borrow().cmp(key)
    }
}

impl<K, V, S> SemigroupRbData<K, V, S> {
    fn value(&self) -> &V {
        &self.value
    }

    fn into_value(self) -> V {
        self.value
    }

    fn semigroup_value(&self) -> &S {
        &self.semigroup_value
    }

    fn into_key_value(self) -> (K, V) {
        (self.key, self.value)
    }

    pub(super) fn data(&self) -> (&K, &V, &S) {
        (&self.key, &self.value, &self.semigroup_value)
    }

    pub(super) fn data_with_mut_value(&mut self) -> (&K, &mut V, &S) {
        (&self.key, &mut self.value, &self.semigroup_value)
    }

    pub(super) fn into_data(self) -> (K, V, S) {
        (self.key, self.value, self.semigroup_value)
    }
}

#[derive(Clone)]
pub struct SemigroupRbTree<K, V, S>(pub(super) base::RedBlackTree<SemigroupRbData<K, V, S>>);

impl<K, V, S> Default for SemigroupRbTree<K, V, S> {
    fn default() -> Self {
        Self(base::RedBlackTree::default())
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S> {
    pub fn new() -> Self {
        Self(base::RedBlackTree::new())
    }

    pub fn cursor(&self) -> Cursor<'_, K, V, S> {
        Cursor::new(self.0.cursor())
    }

    pub fn cursor_mut(&mut self) -> CursorMut<'_, K, V, S> {
        CursorMut::new(self.0.cursor_mut())
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    /// Recomputes the semigroup value of the current node.
    fn on_subtree_change(cursor: &mut base::CursorMut<'_, SemigroupRbData<K, V, S>>) {
        let Some(data) = cursor.get() else { return; };
        let new_semigroup_value = S::op(
            &data.key,
            cursor.peek_left().map(SemigroupRbData::semigroup_value),
            cursor.peek_right().map(SemigroupRbData::semigroup_value),
        );
        cursor.get_mut().unwrap().semigroup_value = new_semigroup_value;
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let semigroup_value = S::op(&key, None, None);
        let data = SemigroupRbData {
            key,
            value,
            semigroup_value,
        };
        self.0.insert(data, Self::on_subtree_change)
            .map(SemigroupRbData::into_value)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.remove(key, Self::on_subtree_change)
            .map(SemigroupRbData::into_key_value)
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    K: Ord,
{
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.contains_key(key)
    }

    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.get(key)
            .map(SemigroupRbData::value)
    }

    pub fn pred_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.pred_data(key).map(|(k, ..)| k)
    }

    pub fn pred_data<Q>(&self, key: &Q) -> Option<(&K, &V, &S)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.pred(key)
            .map(SemigroupRbData::data)
    }

    pub fn pred_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V, &S)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.pred_mut(key)
            .map(SemigroupRbData::data_with_mut_value)
    }

    pub fn succ_key<Q>(&self, key: &Q) -> Option<&K>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.succ_data(key).map(|(k, ..)| k)
    }

    pub fn succ_data<Q>(&self, key: &Q) -> Option<(&K, &V, &S)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.succ(key)
            .map(SemigroupRbData::data)
    }

    pub fn succ_data_with_mut_value<Q>(&mut self, key: &Q) -> Option<(&K, &mut V, &S)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.0.succ_mut(key)
            .map(SemigroupRbData::data_with_mut_value)
    }
}

impl<K, V, S> Extend<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (k, v) in iter {
            self.insert(k, v);
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<'t, K, V, S> IntoIterator for &'t SemigroupRbTree<K, V, S> {
    type Item = (&'t K, &'t V, &'t S);
    type IntoIter = InorderIter<'t, K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter()
    }
}

impl<'t, K, V, S> IntoIterator for &'t mut SemigroupRbTree<K, V, S> {
    type Item = (&'t K, &'t mut V, &'t S);
    type IntoIter = InorderIterMut<'t, K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.inorder_iter_mut()
    }
}

impl<K, V, S> IntoIterator for SemigroupRbTree<K, V, S> {
    type Item = (K, V, S);
    type IntoIter = IntoInorderIter<K, V, S>;

    fn into_iter(self) -> Self::IntoIter {
        self.into_inorder_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::binary_trees::{Neighborhood, binary_search_trees::semigroup_rb_tree::{CanonInterval, CanonSubset, Cursor, Height}, binary_tree_cursor::{BinaryTreeCursor, PeekingCursor}};
    
    use std::fmt::Debug;

    fn assert_semigroup<K, V, S>(tree: &SemigroupRbTree<K, V, S>)
    where 
        S: TreeSemigroup<K> + Debug + PartialEq,
    {
        fn assert_semigroup_recursive<K, V, S>(cursor: Cursor<'_, K, V, S>)
        where 
            S: TreeSemigroup<K> + Debug + PartialEq,
        {
            let Some((k, _, s)) = cursor.get() else { return; };
            let Neighborhood { left, right, .. } = cursor.peek_neighborhood();
            assert_eq!(
                *s,
                S::op(k, left.map(|(.., s)| s), right.map(|(.., s)| s))
            );
            
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            left_cursor.move_left();
            right_cursor.move_right();
            assert_semigroup_recursive(left_cursor);
            assert_semigroup_recursive(right_cursor);
        }

        assert_semigroup_recursive(tree.cursor());
    }

    fn assert_semigroup_tuple<K, V, S1, S2>(tree: &SemigroupRbTree<K, V, (S1, S2)>)
    where 
        S1: TreeSemigroup<K> + Debug + PartialEq,
        S2: TreeSemigroup<K> + Debug + PartialEq,
    {
        fn assert_semigroup_tuple_recursive<K, V, S1, S2>(cursor: Cursor<'_, K, V, (S1, S2)>)
        where 
            S1: TreeSemigroup<K> + Debug + PartialEq,
            S2: TreeSemigroup<K> + Debug + PartialEq,
        {
            let Neighborhood { node: Some((key, .., s)), left, right, .. } = cursor.peek_neighborhood() else { return; };
            let left_semigroup_1 = left.map(|(.., (s1, _))| s1);
            let left_semigroup_2 = left.map(|(.., (_, s2))| s2);
            let right_semigroup_1 = right.map(|(.., (s1, _))| s1);
            let right_semigroup_2 = right.map(|(.., (_, s2))| s2);
            assert_eq!(
                s.0,
                S1::op(key, left_semigroup_1, right_semigroup_1)
            );
            assert_eq!(
                s.1,
                S2::op(key, left_semigroup_2, right_semigroup_2)
            );
            
            let mut left_cursor = cursor;
            let mut right_cursor = cursor.clone();
            left_cursor.move_left();
            right_cursor.move_right();
            assert_semigroup_tuple_recursive(left_cursor);
            assert_semigroup_tuple_recursive(right_cursor);
        }

        assert_semigroup_tuple_recursive(tree.cursor());
    }

    #[test]
    fn test_semigroup_tree() {
        let mut tree = ('a'..='z').map(|c| (c, ()))
            .collect::<SemigroupRbTree<_, _, Height>>();
        assert_semigroup(&tree);
        tree.remove_entry(&'k');
        tree.remove_entry(&'l');
        tree.remove_entry(&'m');
        assert_semigroup(&tree);

        let mut tree = (1..=30).map(|i| (i, ()))
            .collect::<SemigroupRbTree<_, _, CanonInterval<i32>>>();
        assert_semigroup(&tree);
        tree.remove_entry(&5);
        tree.remove_entry(&24);
        tree.remove_entry(&12);
        assert_semigroup(&tree);
        
        let mut tree = (1..=30).map(|i| (i, ()))
            .collect::<SemigroupRbTree<_, _, (Height, CanonSubset<i32>)>>();
        assert_semigroup(&tree);
        assert_semigroup_tuple(&tree);
        tree.remove_entry(&5);
        tree.remove_entry(&24);
        tree.remove_entry(&12);
        assert_semigroup(&tree);
        assert_semigroup_tuple(&tree);
    }
}
