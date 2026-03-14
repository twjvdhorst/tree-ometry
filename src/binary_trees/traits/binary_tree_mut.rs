use super::{BinaryTree, BinaryTreeNode};
use crate::binary_trees::Side;

pub(crate) trait BinaryTreeMut: BinaryTree 
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
}

pub(crate) trait BinaryTreeNodeMut: BinaryTreeNode {
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

    fn detach_left(&mut self) -> Self::Tree;
    fn detach_right(&mut self) -> Self::Tree;
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
}
