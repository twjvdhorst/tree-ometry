use std::cmp::Ordering;

use crate::binary_search_trees::binary_search_tree_node::Side;

pub trait BinaryTreeNode {
    type Tree: BinaryTree;

    fn left_subtree(&self) -> &Self::Tree;
    fn right_subtree(&self) -> &Self::Tree;
    fn subtree(&self, side: Side) -> &Self::Tree {
        match side {
            Side::Left => self.left_subtree(),
            Side::Right => self.right_subtree(),
        }
    }

    fn left_subtree_mut(&mut self) -> &mut Self::Tree;
    fn right_subtree_mut(&mut self) -> &mut Self::Tree;
    fn subtree_mut(&mut self, side: Side) -> &mut Self::Tree {
        match side {
            Side::Left => self.left_subtree_mut(),
            Side::Right => self.right_subtree_mut(),
        }
    }

    fn has_left(&self) -> bool {
        !self.left_subtree().is_leaf()
    }

    fn has_right(&self) -> bool {
        !self.right_subtree().is_leaf()
    }

    fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }

    fn attach_left(&mut self, tree: impl Into<Self::Tree>) -> bool;
    fn attach_right(&mut self, tree: impl Into<Self::Tree>) -> bool;
    fn attach_subtree(&mut self, side: Side, tree: impl Into<Self::Tree>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Self::Tree;
    fn detach_right(&mut self) -> Self::Tree;
    fn detach_subtree(&mut self, side: Side) -> Self::Tree {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree;
    fn replace_right(&mut self, tree: impl Into<Self::Tree>) -> Self::Tree;
    fn replace_subtree(&mut self, side: Side, tree: impl Into<Self::Tree>) -> Self::Tree {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}

pub trait BinaryTree: Sized {
    type Node: BinaryTreeNode<Tree = Self>;

    fn new(root: Self::Node) -> Self;
    fn new_leaf() -> Self;

    fn root(&self) -> Option<&Self::Node>;
    fn root_mut(&mut self) -> Option<&mut Self::Node>;
    
    fn left_subtree(&self) -> Option<&Self> {
        self.root().map(|root| root.left_subtree())
    }

    fn right_subtree(&self) -> Option<&Self> {
        self.root().map(|root| root.right_subtree())
    }

    fn subtree(&self, side: Side) -> Option<&Self> {
        match side {
            Side::Left => self.left_subtree(),
            Side::Right => self.right_subtree(),
        }
    }
    
    fn left_subtree_mut(&mut self) -> Option<&mut Self> {
        self.root_mut().map(|root| root.left_subtree_mut())
    }

    fn right_subtree_mut(&mut self) -> Option<&mut Self> {
        self.root_mut().map(|root| root.right_subtree_mut())
    }

    fn subtree_mut(&mut self, side: Side) -> Option<&mut Self> {
        match side {
            Side::Left => self.left_subtree_mut(),
            Side::Right => self.right_subtree_mut(),
        }
    }

    fn is_leaf(&self) -> bool {
        self.root().is_none()
    }

    fn has_left(&self) -> bool {
        self.root()
            .map(|root| !root.left_subtree().is_leaf())
            .unwrap_or(false)
    }

    fn has_right(&self) -> bool {
        self.root()
            .map(|root| !root.right_subtree().is_leaf())
            .unwrap_or(false)
    }

    fn has_child(&self, side: Side) -> bool {
        match side {
            Side::Left => self.has_left(),
            Side::Right => self.has_right(),
        }
    }
    
    fn attach_left(&mut self, tree: impl Into<Self>) -> bool {
        let Some(root) = self.root_mut() else { return false; };
        root.attach_left(tree)
    }

    fn attach_right(&mut self, tree: impl Into<Self>) -> bool {
        let Some(root) = self.root_mut() else { return false; };
        root.attach_right(tree)
    }

    fn attach_subtree(&mut self, side: Side, tree: impl Into<Self>) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }
    
    fn detach_left(&mut self) -> Option<Self> {
        Some(self.root_mut()?.detach_left())
    }

    fn detach_right(&mut self) -> Option<Self> {
        Some(self.root_mut()?.detach_right())
    }

    fn detach_subtree(&mut self, side: Side) -> Option<Self> {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }
    
    fn replace_left(&mut self, tree: impl Into<Self>) -> Option<Self> {
        Some(self.root_mut()?.replace_left(tree))
    }

    fn replace_right(&mut self, tree: impl Into<Self>) -> Option<Self> {
        Some(self.root_mut()?.replace_right(tree))
    }

    fn replace_subtree(&mut self, side: Side, tree: impl Into<Self>) -> Option<Self> {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }
}

pub trait BinarySearchTreeNode {
    type Key: Ord;
    type Value;

    fn key(&self) -> &Self::Key;
    fn value(&self) -> &Self::Value;
    fn value_mut(&mut self) -> &mut Self::Value;
}

pub trait BinarySearchTree: BinaryTree
where 
    Self::Node: BinarySearchTreeNode,
{
    fn predecessor<Q>(&self, value: &Q) -> Option<&<Self::Node as BinarySearchTreeNode>::Key>
    where
        <Self::Node as BinarySearchTreeNode>::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let root = self.root()?;
        match Q::cmp(value, root.key().as_ref()) {
            Ordering::Equal => Some(root.key()),
            Ordering::Less => self.left_subtree()?.predecessor(value),
            Ordering::Greater => self.right_subtree()?.predecessor(value)
                .or(Some(root.key())),
        }
    }
        
    fn successor<Q>(&self, value: &Q) -> Option<&<Self::Node as BinarySearchTreeNode>::Key>
    where
        <Self::Node as BinarySearchTreeNode>::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let root = self.root()?;
        match Q::cmp(value, root.key().as_ref()) {
            Ordering::Equal => Some(root.key()),
            Ordering::Greater => self.right_subtree()?.successor(value),
            Ordering::Less => self.left_subtree()?.successor(value)
                .or(Some(root.key())),
        }
    }
        
    fn get<Q>(&self, value: &Q) -> Option<&<Self::Node as BinarySearchTreeNode>::Key>
    where
        <Self::Node as BinarySearchTreeNode>::Key: AsRef<Q>,
        Q: Ord + ?Sized,
    {
        let root = self.root()?;
        match Q::cmp(value, root.key().as_ref()) {
            Ordering::Equal => Some(root.key()),
            Ordering::Greater => self.right_subtree()?.get(value),
            Ordering::Less => self.left_subtree()?.get(value),
        }
    }
}

pub trait Insert: BinarySearchTree
where 
    Self::Node: BinarySearchTreeNode,
{
    fn insert(&mut self, key: <Self::Node as BinarySearchTreeNode>::Key, value: <Self::Node as BinarySearchTreeNode>::Value) -> Option<<Self::Node as BinarySearchTreeNode>::Value>;
}
