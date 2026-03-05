use crate::binary_trees::Side;

pub trait BinaryTree {
    type Node: BinaryTreeNode;

    fn new_leaf() -> Self;
    fn is_leaf(&self) -> bool;
    fn root(&self) -> Option<&Self::Node>;
}

pub trait BinaryTreeMut: BinaryTree {
    fn root_mut(&mut self) -> Option<&mut Self::Node>;
    fn into_root(self) -> Option<Self::Node>;
}

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
    fn subtrees(&self) -> (&Self::Tree, &Self::Tree) {
        (self.left_subtree(), self.right_subtree())
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