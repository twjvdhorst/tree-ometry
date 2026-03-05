use std::borrow::Borrow;
use std::fmt;

use crate::binary_trees::red_black_trees::red_black_node::RedBlackNode;
use crate::binary_trees::traits::{
    BinaryTree, BinaryTreeMut, BinaryTreeNode, Dynamic
};

pub struct RedBlackTree<K, V>(Option<RedBlackNode<K, V, Self>>);

impl<K, V> Default for RedBlackTree<K, V> {
    fn default() -> Self {
        Self::new_leaf()
    }
}

impl<K, V> RedBlackTree<K, V> {
    pub fn new() -> Self {
        Self::new_leaf()
    }
}

impl<K, V> From<RedBlackNode<K, V, Self>> for RedBlackTree<K, V> {
    fn from(value: RedBlackNode<K, V, Self>) -> Self {
        Self(Some(value))
    }
}

impl<K, V> BinaryTree for RedBlackTree<K, V> {
    type Node = RedBlackNode<K, V, Self>;

    fn new_leaf() -> Self {
        Self(None)
    }

    fn is_leaf(&self) -> bool {
        self.0.is_none()
    }

    fn root(&self) -> Option<&Self::Node> {
        self.0.as_ref()
    }
}

impl<K, V> BinaryTreeMut for RedBlackTree<K, V> {
    fn root_mut(&mut self) -> Option<&mut Self::Node> {
        self.0.as_mut()
    }

    fn into_root(self) -> Option<Self::Node> {
        self.0
    }
}

impl<K, V> Dynamic for RedBlackTree<K, V>
where
    K: Ord,
{
    type Key = K;
    type Value = V;

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        RedBlackNode::insert(self, key, value)
    }

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(K, V)>
    where 
        K: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        RedBlackNode::remove_entry(self, key)
    }
}

impl<K, V> Extend<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn extend<T: IntoIterator<Item = (K, V)>>(&mut self, iter: T) {
        for (key, value) in iter {
            self.insert(key, value);
        }
    }
}

impl<K, V> FromIterator<(K, V)> for RedBlackTree<K, V>
where 
    K: Ord,
{
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut tree = Self::default();
        tree.extend(iter);
        tree
    }
}

impl<K, V> fmt::Debug for RedBlackTree<K, V>
where 
    K: fmt::Debug,
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V>(tree: &RedBlackTree<K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Debug,
            V: fmt::Debug,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "{root:?}\n")?;
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
    
impl<K, V> fmt::Display for RedBlackTree<K, V>
where 
    K: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recursive_fmt<K, V>(tree: &RedBlackTree<K, V>, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
        where
            K: fmt::Display,
            V: fmt::Display,
        {
            write!(f, "{prefix}")?;
            if is_left {
                write!(f, "├──")?;
            } else {
                write!(f, "└──")?;
            };
            if let Some(root) = tree.root() {
                write!(f, "{root}\n")?;
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
