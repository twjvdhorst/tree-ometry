use std::{borrow::Borrow, cmp::Ordering};

use super::Side;

pub trait BinaryTree {
    type Node;

    fn is_leaf(&self) -> bool;
    fn root(&self) -> Option<&Self::Node>;

    fn left_subtree(&self) -> Option<&Self>;
    fn right_subtree(&self) -> Option<&Self>;
    fn subtree(&self, side: Side) -> Option<&Self> {
        match side {
            Side::Left => self.left_subtree(),
            Side::Right => self.right_subtree(),
        }
    }
    fn subtrees(&self) -> Option<(&Self, &Self)> {
        Option::zip(self.left_subtree(), self.right_subtree())
    }
}

pub(crate) trait BinaryTreeMut: BinaryTree + Sized {
    fn root_mut(&mut self) -> Option<&mut Self::Node>;

    fn left_subtree_mut(&mut self) -> Option<&mut Self>;
    fn right_subtree_mut(&mut self) -> Option<&mut Self>;
    fn subtree_mut(&mut self, side: Side) -> Option<&mut Self> {
        match side {
            Side::Left => self.left_subtree_mut(),
            Side::Right => self.right_subtree_mut(),
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self>) -> bool;
    fn attach_subtree(&mut self, side: Side, tree: impl Into<Self>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }

    fn detach_left(&mut self) -> Option<Self>;
    fn detach_right(&mut self) -> Option<Self>;
    fn detach_subtree(&mut self, side: Side) -> Option<Self> {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }
    fn detach_both(&mut self) -> Option<(Self, Self)> {
        Option::zip(self.detach_left(), self.detach_right())
    }

    fn replace_left(&mut self, tree: impl Into<Self>) -> Option<Self>;
    fn replace_right(&mut self, tree: impl Into<Self>) -> Option<Self>;
    fn replace_subtree(&mut self, side: Side, tree: impl Into<Self>) -> Option<Self> {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }

    /// Rotates the left edge, making the left child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_left(&mut self) -> bool {
        let Some(mut new_tree) = self.detach_left() else { return false; };
        if let Some(rotating_subtree) = new_tree.detach_right() {
            self.replace_left(rotating_subtree);
            std::mem::swap(self, &mut new_tree);
            self.replace_right(new_tree);
            true
        } else {
            // Left subtree is a leaf.
            false
        }
    }

    /// Rotates the right edge, making the right child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_right(&mut self) -> bool {
        let Some(mut new_tree) = self.detach_right() else { return false; };
        if let Some(rotating_subtree) = new_tree.detach_left() {
            self.replace_right(rotating_subtree);
            std::mem::swap(self, &mut new_tree);
            self.replace_left(new_tree);
            true
        } else {
            // Right subtree is a leaf.
            false
        }
    }

    fn rotate_edge(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.rotate_left(),
            Side::Right => self.rotate_right(),
        }
    }
}

pub trait BinarySearchTree: BinaryTree {
    type Key: Ord;
    type Value;

    #[inline]
    fn key(&self) -> Option<&Self::Key> {
        self.data().map(|(k, _)| k)
    }
    
    #[inline]
    fn value(&self) -> Option<&Self::Value> {
        self.data().map(|(_, v)| v)
    }

    fn data(&self) -> Option<(&Self::Key, &Self::Value)>;

    fn predecessor<Q>(&self, key: &Q) -> Option<&Self::Key>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut pred_key = None;
        while let Some(curr_key) = current.key() {
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Equal => return Some(curr_key),
                Ordering::Less => current = current.left_subtree()?,
                Ordering::Greater => {
                    pred_key = Some(curr_key);
                    current = current.right_subtree()?;
                },
            }
        }
        pred_key
    }

    fn successor<Q>(&self, key: &Q) -> Option<&Self::Key>
    where
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        let mut current = self;
        let mut pred_key = None;
        while let Some(curr_key) = current.key() {
            match Q::cmp(key, curr_key.borrow()) {
                Ordering::Equal => return Some(curr_key),
                Ordering::Less => {
                    pred_key = Some(curr_key);
                    current = current.left_subtree()?;
                },
                Ordering::Greater =>  current = current.right_subtree()?,
            }
        }
        pred_key
    }

    fn min(&self) -> Option<&Self::Key> {
        let mut current = self;
        while let Some(child) = current.left_subtree()
            && !child.is_leaf()
        {
            current = child;
        }
        current.key()
    }

    fn max(&self) -> Option<&Self::Key> {
        let mut current = self;
        while let Some(child) = current.right_subtree()
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
        self.get(key).is_some()
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
        let root_key = self.key()?;
        match Q::cmp(key, root_key.borrow()) {
            Ordering::Equal => self.data(),
            Ordering::Greater => self.right_subtree()?.get_key_value(key),
            Ordering::Less => self.left_subtree()?.get_key_value(key),
        }
    }
}
