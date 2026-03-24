use super::{BinaryTree, BinaryTreeNode};
use crate::binary_trees::Side;

pub trait BinaryTreeMut: BinaryTree 
where 
    Self::Node: BinaryTreeNodeMut,
{
    fn root_mut(&mut self) -> Option<&mut Self::Node>;
    fn into_root(self) -> Option<Self::Node>;

    fn left_subtree_mut(&mut self) -> Option<&mut Self> {
        self.root_mut().map(BinaryTreeNodeMut::left_subtree_mut)
    }

    fn right_subtree_mut(&mut self) -> Option<&mut Self> {
        self.root_mut().map(BinaryTreeNodeMut::right_subtree_mut)
    }

    fn subtree_mut(&mut self, side: Side) -> Option<&mut Self> {
        self.root_mut().map(|root| root.subtree_mut(side))
    }

    fn subtrees_mut(&mut self) -> Option<(&mut Self, &mut Self)> {
        self.root_mut().map(BinaryTreeNodeMut::subtrees_mut)
    }

    fn attach_left(&mut self, tree: Self) -> bool
    where 
        Self: Sized,
    {
        if let Some(root) = self.root_mut() {
            root.attach_left(tree)
        } else { false }
    }

    fn attach_right(&mut self, tree: Self) -> bool
    where 
        Self: Sized,
    {
        if let Some(root) = self.root_mut() {
            root.attach_right(tree)
        } else { false }
    }
    
    fn attach_subtree(&mut self, side: Side, tree: Self) -> bool
    where 
        Self: Sized,
    {
        if let Some(root) = self.root_mut() {
            root.attach_subtree(side, tree)
        } else { false }
    }

    fn detach_left(&mut self) -> Option<Self>
    where 
        Self: Sized,
    {
        self.root_mut().map(BinaryTreeNodeMut::detach_left)
    }
    
    fn detach_right(&mut self) -> Option<Self>
    where 
        Self: Sized,
    {
        self.root_mut().map(BinaryTreeNodeMut::detach_right)
    }

    fn detach_subtree(&mut self, side: Side) -> Option<Self>
    where 
        Self: Sized,
    {
        self.root_mut().map(|root| root.detach_subtree(side))
    }

    fn detach_both(&mut self) -> Option<(Self, Self)>
    where 
        Self: Sized,
    {
        self.root_mut().map(BinaryTreeNodeMut::detach_both)
    }

    fn replace_left(&mut self, tree: Self) -> Option<Self>
    where 
        Self: Sized,
    {
        Some(self.root_mut()?.replace_left(tree))
    }
    
    fn replace_right(&mut self, tree: Self) -> Option<Self>
    where 
        Self: Sized,
    {
        Some(self.root_mut()?.replace_right(tree))
    }

    fn replace_subtree(&mut self, side: Side, tree: Self) -> Option<Self>
    where 
        Self: Sized,
    {
        Some(self.root_mut()?.replace_subtree(side, tree))
    }

    fn rotate_left(&mut self) -> bool {
        if let Some(root) = self.root_mut() {
            root.rotate_left()
        } else {
            false
        }
    }

    fn rotate_right(&mut self) -> bool {
        if let Some(root) = self.root_mut() {
            root.rotate_right()
        } else {
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

pub trait BinaryTreeNodeMut: BinaryTreeNode
where 
    Self::Tree: BinaryTreeMut,
{
    fn left_subtree_mut(&mut self) -> &mut Self::Tree;
    fn right_subtree_mut(&mut self) -> &mut Self::Tree;
    fn subtree_mut(&mut self, side: Side) -> &mut Self::Tree {
        match side {
            Side::Left => self.left_subtree_mut(),
            Side::Right => self.right_subtree_mut(),
        }
    }
    fn subtrees_mut(&mut self) -> (&mut Self::Tree, &mut Self::Tree);

    fn attach_left(&mut self, tree: Self::Tree) -> bool;
    fn attach_right(&mut self, tree: Self::Tree) -> bool;
    fn attach_subtree(&mut self, side: Side, tree: Self::Tree) -> bool {
        match side {
            Side::Left => self.attach_left(tree),
            Side::Right => self.attach_right(tree),
        }
    }

    fn detach_left(&mut self) -> Self::Tree {
        self.replace_left(Self::Tree::new_leaf())
    }

    fn detach_right(&mut self) -> Self::Tree {
        self.replace_right(Self::Tree::new_leaf())
    }

    fn detach_subtree(&mut self, side: Side) -> Self::Tree {
        match side {
            Side::Left => self.detach_left(),
            Side::Right => self.detach_right(),
        }
    }

    fn detach_both(&mut self) -> (Self::Tree, Self::Tree) {
        (self.detach_left(), self.detach_right())
    }
    
    fn replace_left(&mut self, tree: Self::Tree) -> Self::Tree;
    fn replace_right(&mut self, tree: Self::Tree) -> Self::Tree;
    fn replace_subtree(&mut self, side: Side, tree: Self::Tree) -> Self::Tree {
        match side {
            Side::Left => self.replace_left(tree),
            Side::Right => self.replace_right(tree),
        }
    }

    /// Rotates the left edge, making the left child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_left(&mut self) -> bool
    where
        Self: Sized,
    {
        let mut new_tree = self.detach_left();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_right();
            self.replace_left(rotating_subtree);
            std::mem::swap(self, &mut new_root);
            self.replace_right(new_tree);
            true
        } else {
            // Left subtree is a leaf.
            false
        }
    }

    /// Rotates the right edge, making the right child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_right(&mut self) -> bool
    where
        Self: Sized,
    {
        let mut new_tree = self.detach_right();
        if let Some(mut new_root) = new_tree.root_mut() {
            let rotating_subtree = new_root.detach_left();
            self.replace_right(rotating_subtree);
            std::mem::swap(self, &mut new_root);
            self.replace_left(new_tree);
            true
        } else {
            // Right subtree is a leaf.
            false
        }
    }

    fn rotate_edge(&mut self, side: Side) -> bool
    where
        Self: Sized,
    {
        match side {
            Side::Left => self.rotate_left(),
            Side::Right => self.rotate_right(),
        }
    }
}
