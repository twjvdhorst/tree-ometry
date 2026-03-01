use std::{borrow::Borrow, fmt};

use lending_iterator::LendingIterator;

use super::tree_semigroup::TreeSemigroup;
use crate::binary_trees::{
    red_black_trees::red_black_node::RedBlackNode, 
    traits::{
        BinaryTree, 
        BinaryTreeNode, 
        Dynamic,
        iterable_postorder::IterablePostorderMut,
    },
};

pub struct SemigroupRbNode<K, V, S, T> {
    node: RedBlackNode<K, V, T>,
    semigroup_value: S,
    accessed_mut: bool,
}

pub struct SemigroupRbTree<K, V, S>(Option<SemigroupRbNode<K, V, S, Self>>);

impl<K, V, S> Default for SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K>,
{
    fn default() -> Self {
        Self::new_leaf()
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K>,
{
    pub fn new() -> Self {
        Self::new_leaf()
    }

    fn semigroup_value(&self) -> Option<&S> {
        Some(&self.0.as_ref()?.semigroup_value)
    }

    fn set_semigroup_value(&mut self, new_data: S) {
        if let Some(root) = self.0.as_mut() {
            root.semigroup_value = new_data
        }
    }

    fn mark_unaccessed(&mut self) {
        if let Some(root) = self.0.as_mut() {
            root.accessed_mut = false;
        }
    }

    fn mark_accessed(&mut self) {
        if let Some(root) = self.0.as_mut() {
            root.accessed_mut = true;
        }
    }

    /// Updates semigroup values in a bottom-up fashion.
    /// Considers only tree nodes that have been accessed mutably,
    /// as others have their subtree, and thus semigroup value, intact.
    fn update_semigroup_values(&mut self) {
        let mut changed_trees_iter = self.postorder_iter_filtered_mut(|tree|
            tree.0.as_ref().map(|root| root.accessed_mut) == Some(true)
        );
        while let Some(tree) = changed_trees_iter.next() {
            let Some(root) = tree.root() else { continue; };
            let (left, right) = root.subtrees();
            tree.set_semigroup_value(
                S::op(root.key(), left.semigroup_value(), right.semigroup_value())
            );
            tree.mark_unaccessed();
        }
    }
}

impl<K, V, S> BinaryTree for SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K>,
{
    type Node = RedBlackNode<K, V, Self>;

    fn new_leaf() -> Self {
        Self(None)
    }

    fn new_node(node: Self::Node) -> Self {
        let semigroup_value = S::op(node.key(), None, None);
        Self(Some(SemigroupRbNode {
            node, 
            semigroup_value,
            accessed_mut: false
        }))
    }

    fn is_leaf(&self) -> bool {
        self.0.is_none()
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref().map(|root| &root.node)
    }

    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        let root = self.0.as_mut()?;
        root.accessed_mut = true;
        Some(&mut root.node)
    }

    fn into_root(self) -> Option<Self::Node> {
        self.0.map(|root| root.node)
    }
}

impl<K, V, S> Dynamic for SemigroupRbTree<K, V, S>
where
    K: Ord,
    S: TreeSemigroup<K>,
{
    type Key = K;
    type Value = V;

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        let result = RedBlackNode::insert(self, key, value);
        self.update_semigroup_values();
        result
    }

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let result = RedBlackNode::remove_entry(self, key);
        self.update_semigroup_values();
        result
    }
}

impl<K, V, S> Extend<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        // To save on semigroup calculations, first perform the insertions without updating semigroup values, then do single update pass.
        for (key, value) in iter {
            <Self as Dynamic>::insert(self, key, value);
        }
        self.update_semigroup_values();
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

impl<K, V, S> fmt::Debug for SemigroupRbTree<K, V, S>
where 
    K: fmt::Debug,
    V: fmt::Debug,
    S: fmt::Debug + TreeSemigroup<K>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V, S>(tree: &SemigroupRbTree<K, V, S>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Debug,
            V: fmt::Debug,
            S: fmt::Debug + TreeSemigroup<K>,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "({root:?}, {:?})\n", tree.semigroup_value().unwrap())?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                recursive_fmt(root.left_subtree(), f, &new_prefix, true)?;
                recursive_fmt(root.right_subtree(), f, &new_prefix, false)?;
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
        
        write!(f, "\n")?;
        recursive_fmt(self, f, "", false)
    }
}
    
impl<K, V, S> fmt::Display for SemigroupRbTree<K, V, S>
where 
    K: fmt::Display,
    V: fmt::Display,
    S: fmt::Display + TreeSemigroup<K>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V, S>(tree: &SemigroupRbTree<K, V, S>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Display,
            V: fmt::Display,
            S: fmt::Display + TreeSemigroup<K>,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "({root}, {})\n", tree.semigroup_value().unwrap())?;
                let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                recursive_fmt(root.left_subtree(), f, &new_prefix, true)?;
                recursive_fmt(root.right_subtree(), f, &new_prefix, false)?;
                Ok(())
            } else {
                write!(f, "L\n")
            }
        }
        
        write!(f, "\n")?;
        recursive_fmt(self, f, "", false)
    }
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;
    use super::*;
    use crate::binary_trees::red_black_trees::semigroup::*;

    fn assert_semigroup<K, V, S>(tree: &SemigroupRbTree<K, V, S>)
    where 
        S: TreeSemigroup<K> + Debug + PartialEq,
    {
        let Some(root) = tree.root() else { return; };
        let (left, right) = root.subtrees();
        assert_semigroup(left);
        assert_semigroup(right);
        assert_eq!(
            *tree.semigroup_value().unwrap(),
            S::op(root.key(), left.semigroup_value(), right.semigroup_value())
        );
    }

    #[test]
    fn test_semigroup_tree() {
        let mut tree = ('a'..='z').map(|c| (c, ()))
            .collect::<SemigroupRbTree<_, _, Height>>();
        assert_semigroup(&tree);
        tree.remove(&'k');
        tree.remove(&'l');
        tree.remove(&'m');
        assert_semigroup(&tree);

        let mut tree = (1..=30).map(|i| (i, ()))
            .collect::<SemigroupRbTree<_, _, CanonInterval<i32>>>();
        assert_semigroup(&tree);
        assert_eq!(tree.semigroup_value(), Some(&(1, 30).into()));
        tree.remove(&5);
        tree.remove(&24);
        assert_semigroup(&tree);
    }
}
