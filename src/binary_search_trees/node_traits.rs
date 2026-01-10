use std::cmp::Ordering;

use crate::binary_search_trees::binary_search_tree_node::Side;

pub trait BinaryTreeNode {
    type Tree;

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
}

pub trait BinaryTree {
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
        self.root_mut().map(|root| root.left_subtree_mut())
    }

    fn subtree_mut(&mut self, side: Side) -> Option<&mut Self> {
        match side {
            Side::Left => self.left_subtree_mut(),
            Side::Right => self.right_subtree_mut(),
        }
    }

    fn is_leaf(&self) -> bool { self.root().is_none() }
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
