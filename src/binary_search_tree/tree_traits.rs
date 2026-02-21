use std::cmp::Ordering;

use super::Side;

pub trait BinaryTree {
    type NodeRef<'tree>
    where Self: 'tree;

    fn is_leaf(&self) -> bool;
    fn node_ref(&'_ self) -> Option<Self::NodeRef<'_>>;

    fn left_subtree(&self) -> Option<&Self>;
    fn right_subtree(&self) -> Option<&Self>;
    fn subtree(&self, side: Side) -> Option<&Self> {
        match side {
            Side::Left => self.left_subtree(),
            Side::Right => self.right_subtree(),
        }
    }
}

pub(crate) trait BinaryTreeMut: BinaryTree + Sized {
    type NodeRefMut<'tree>
    where Self: 'tree;

    fn node_ref_mut(&'_ mut self) -> Option<Self::NodeRefMut<'_>>;

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
    
    fn replace_left(&mut self, tree: impl Into<Self>) -> Option<Self>;
    fn replace_right(&mut self, tree: impl Into<Self>) -> Option<Self>;
    fn replace_subtree(&mut self, side: Side, tree: impl Into<Self>) -> Option<Self> {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }

    /// Performs a left tree rotation, changing self to point to the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_left(&mut self) -> bool {
        let Some(mut new_tree) = self.detach_right() else { return false; };
        if let Some(rotating_subtree) = new_tree.detach_left() {
            self.replace_right(rotating_subtree);
        }
        std::mem::swap(self, &mut new_tree);
        self.replace_left(new_tree);
        true
    }

    /// Performs a right tree rotation, changing self to point to the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_right(&mut self) -> bool {
        let Some(mut new_tree) = self.detach_left() else { return false; };
        if let Some(rotating_subtree) = new_tree.detach_right() {
            self.replace_left(rotating_subtree);
        }
        std::mem::swap(self, &mut new_tree);
        self.replace_right(new_tree);
        true
    }
}

pub trait BinarySearchTree: BinaryTree {
    type Key: Ord;
    type Value;

    fn key(&self) -> Option<&Self::Key>;
    fn value(&self) -> Option<&Self::Value>;

    fn predecessor<Q>(&self, value: &Q) -> Option<&Self::Key>
    where
        Self::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let key = self.key()?;
        match Q::cmp(value, key.as_ref()) {
            Ordering::Equal => Some(key),
            Ordering::Less => self.left_subtree()?.predecessor(value),
            Ordering::Greater => self.right_subtree()
                .and_then(|right| right.predecessor(value))
                .or(Some(key)),
        }
    }

    fn successor<Q>(&self, value: &Q) -> Option<&Self::Key>
    where
        Self::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let key = self.key()?;
        match Q::cmp(value, key.as_ref()) {
            Ordering::Equal => Some(key),
            Ordering::Greater => self.right_subtree()?.successor(value),
            Ordering::Less => self.left_subtree()
                .and_then(|left| left.successor(value))
                .or(Some(key)),
        }
    }

    fn get<Q>(&self, value: &Q) -> Option<&Self::Value>
    where
        Self::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let key = self.key()?;
        match Q::cmp(value, key.as_ref()) {
            Ordering::Equal => self.value(),
            Ordering::Greater => self.right_subtree()?.get(value),
            Ordering::Less => self.left_subtree()?.get(value),
        }
    }
}
