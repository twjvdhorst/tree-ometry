use std::{borrow::Borrow, cmp::Ordering};

use crate::binary_trees::binary_tree_traits::{BinaryTree, BinaryTreeNode};

pub trait BinarySearchTreeNode: BinaryTreeNode {
    type Key: Ord;
    type Value;

    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
}

pub trait BinarySearchTree: BinaryTree
where 
    Self::Node: BinaryTreeNode<Tree = Self>,
{
    type Key: Ord;
    type Value;

    fn key(&self) -> Option<&Self::Key>;
    fn value(&self) -> Option<&Self::Value>;

    fn pred_key<Q>(&self, key: &Q) -> Option<&Self::Key>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut pred_key = None;
        loop {
            let Some(curr_key) = current.key() else { break; };
            let Some(root) = current.root() else { break; };
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Equal => return Some(curr_key),
                Ordering::Less => current = root.left_subtree(),
                Ordering::Greater => {
                    pred_key = Some(curr_key);
                    current = root.right_subtree();
                },
            }
        }
        pred_key
    }

    fn succ_key<Q>(&self, key: &Q) -> Option<&Self::Key>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut succ_key = None;
        loop {
            let Some(curr_key) = current.key() else { break; };
            let Some(root) = current.root() else { break; };
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Equal => return Some(curr_key),
                Ordering::Less => {
                    succ_key = Some(curr_key);
                    current = root.left_subtree();
                },
                Ordering::Greater => current = root.right_subtree(),
            }
        }
        succ_key
    }

    fn min_key(&self) -> Option<&Self::Key> {
        let mut current = self;
        while let Some(child) = current.root().map(BinaryTreeNode::left_subtree) 
            && !child.is_leaf()
        {
            current = child;
        }
        current.key()
    }

    fn max_key(&self) -> Option<&Self::Key> {
        let mut current = self;
        while let Some(child) = current.root().map(BinaryTreeNode::right_subtree) 
            && !child.is_leaf()
        {
            current = child;
        }
        current.key()
    }

    #[inline]
    fn contains_key<Q>(&self, key: &Q) -> bool
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_key_value(key).is_some()
    }

    #[inline]
    fn get<Q>(&self, key: &Q) -> Option<&Self::Value>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        self.get_key_value(key).map(|(_, v)| v)
    }

    fn get_key_value<Q>(&self, key: &Q) -> Option<(&Self::Key, &Self::Value)>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        while let Some(root) = current.root() {
            let (curr_key, curr_value) = Option::zip(current.key(), current.value())?;
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Equal => return Some((curr_key, curr_value)),
                Ordering::Less => current = root.left_subtree(),
                Ordering::Greater => current = root.right_subtree(),
            }
        }
        None
    }
}

impl<T> BinarySearchTree for T
where 
    T: BinaryTree<Node: BinarySearchTreeNode<Tree = T>>,
{
    type Key = <Self::Node as BinarySearchTreeNode>::Key;
    type Value = <Self::Node as BinarySearchTreeNode>::Value;

    fn key(&self) -> Option<&Self::Key> {
        self.root().map(BinarySearchTreeNode::key)
    }

    fn value(&self) -> Option<&Self::Value> {
        self.root().map(BinarySearchTreeNode::value)
    }
}
