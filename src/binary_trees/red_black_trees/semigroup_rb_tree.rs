use std::{borrow::Borrow, fmt};

use lending_iterator::LendingIterator;
use paste::paste;

use crate::binary_trees::{binary_tree_traits::{BinaryTree, BinaryTreeNode}, red_black_trees::{red_black_node::RedBlackNode, tree_semigroup::TreeSemigroup}, tree_iterators::{inorder::{InorderIter, InorderIterMut}, postorder::{PostorderIter, PostorderIterMut}, preorder::{PreorderIter, PreorderIterMut}}};

pub struct SemigroupRbNode<K, V, S, T> {
    node: RedBlackNode<K, V, T>,
    semigroup_value: S,
    accessed_mut: bool,
}

pub struct SemigroupRbTree<K, V, S>(Option<SemigroupRbNode<K, V, S, Self>>);

impl<K, V, S> Default for SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K, V>,
{
    fn default() -> Self {
        Self::new_leaf()
    }
}

impl<K, V, S> SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K, V>,
{
    pub fn new() -> Self {
        Self::new_leaf()
    }

    fn get_semigroup_value(&self) -> Option<&S> {
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
}

impl<K, V, S> BinaryTree for SemigroupRbTree<K, V, S>
where 
    S: TreeSemigroup<K, V>,
{
    type Node = RedBlackNode<K, V, Self>;

    fn new_leaf() -> Self {
        Self(None)
    }

    fn new_node(node: Self::Node) -> Self {
        let semigroup_value = S::op(node.key(), node.value(), &S::leaf_val(), &S::leaf_val());
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

impl<K, V, S> SemigroupRbTree<K, V, S>
where
    K: Ord,
    S: TreeSemigroup<K, V>,
{
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let result = RedBlackNode::insert(self, key, value);
        self.update_semigroup_values();
        result
    }

    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.remove_entry(key).map(|(_, v)| v)
    }

    pub fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let result = RedBlackNode::remove_entry(self, key);
        self.update_semigroup_values();
        result
    }

    /// Updates semigroup values in a bottom-up fashion.
    /// Considers only tree nodes that have been accessed mutably,
    /// as others have their subtree, and thus semigroup value, intact.
    fn update_semigroup_values(&mut self) {
        let mut changed_trees_iter = self.postorder_iter_mut_filtered(|tree|
            tree.0.as_ref().map(|root| root.accessed_mut) == Some(true)
        );
        while let Some(tree) = changed_trees_iter.next() {
            let Some(root) = tree.root() else { continue; };
            let (left, right) = root.subtrees();
            let new_semigroup_value = match (left.get_semigroup_value(), right.get_semigroup_value()) {
                (Some(x), Some(y)) => S::op(root.key(), root.value(), x, y),
                (Some(x), None) => S::op(root.key(), root.value(), x, &S::leaf_val()),
                (None, Some(y)) => S::op(root.key(), root.value(), &S::leaf_val(), y),
                (None, None) => S::op(root.key(), root.value(), &S::leaf_val(), &S::leaf_val()),
            };
            tree.set_semigroup_value(new_semigroup_value);
            tree.mark_unaccessed();
        }
    }
}

impl<K, V, S> Extend<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K, V>,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V, S> FromIterator<(K, V)> for SemigroupRbTree<K, V, S>
where 
    K: Ord,
    S: TreeSemigroup<K, V>,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

use super::tree_macros::{make_iter, make_iter_mut};
impl<K, V, S> SemigroupRbTree<K, V, S>
where
    S: TreeSemigroup<K, V>,
{
    make_iter!(pub, inorder_iter, InorderIter);
    make_iter_mut!(pub, inorder_iter_mut, InorderIterMut);
    make_iter!(pub, preorder_iter, PreorderIter);
    make_iter_mut!(pub, preorder_iter_mut, PreorderIterMut);
    make_iter!(pub, postorder_iter, PostorderIter);
    make_iter_mut!(pub, postorder_iter_mut, PostorderIterMut);
}

impl<K, V, S> fmt::Debug for SemigroupRbTree<K, V, S>
where 
    K: fmt::Debug,
    V: fmt::Debug,
    S: fmt::Debug + TreeSemigroup<K, V>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V, S>(tree: &SemigroupRbTree<K, V, S>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Debug,
            V: fmt::Debug,
            S: fmt::Debug + TreeSemigroup<K, V>,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "({root:?}, {:?})\n", tree.get_semigroup_value().unwrap())?;
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
    S: fmt::Display + TreeSemigroup<K, V>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V, S>(tree: &SemigroupRbTree<K, V, S>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Display,
            V: fmt::Display,
            S: fmt::Display + TreeSemigroup<K, V>,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "({root}, {})\n", tree.get_semigroup_value().unwrap())?;
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
    use super::*;
    use crate::binary_trees::red_black_trees::tree_semigroup::TreeSemigroup;
    
    struct Max(i32);
    impl<V> TreeSemigroup<i32, V> for Max {
        fn leaf_val() -> Self {
            Self(0)
        }

        fn op(key: &i32, _value: &V, left: &Self, right: &Self) -> Self {
            Self(i32::max(*key, i32::max(left.0, right.0)))
        }
    }
    impl fmt::Display for Max {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[test]
    fn test_semigroup_tree() {
        let tree = (1..=30).map(|i| (i, i % 10))
            .collect::<SemigroupRbTree<_, _, Max>>();
        println!("{tree}");
    }
}
