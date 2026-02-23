use super::Side;

pub trait BinaryTree {
    type Node;

    fn new_leaf() -> Self;
    fn new_node(node: Self::Node) -> Self;

    fn is_leaf(&self) -> bool;

    fn root(&self) -> Option<&Self::Node>;
    fn root_mut(&mut self) -> Option<&mut Self::Node>;
    fn into_root(self) -> Option<Self::Node>;
}

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
    fn subtrees(&self) -> (&Self::Tree, &Self::Tree) {
        (self.left_subtree(), self.right_subtree())
    }
}

pub(crate) trait BinaryTreeNodeMut: BinaryTreeNode + Sized
where 
    Self::Tree: BinaryTree<Node = Self>,
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

    /// Rotates the left edge, making the left child the new root.
    /// Returns a true if the tree was changed (a rotation happened), and false otherwise.
    fn rotate_left(&mut self) -> bool {
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
    fn rotate_right(&mut self) -> bool {
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

    fn rotate_edge(&mut self, side: Side) -> bool {
        match side {
            Side::Left => self.rotate_left(),
            Side::Right => self.rotate_right(),
        }
    }
}
