use std::fmt::{self, Debug, Display};

use crate::binary_trees::Side;

pub trait BinaryTree {
    type Node: BinaryTreeNode<Tree = Self>;

    fn new_leaf() -> Self;
    fn is_leaf(&self) -> bool;
    fn root(&self) -> Option<&Self::Node>;

    fn left_subtree(&self) -> Option<&Self> {
        self.root().map(BinaryTreeNode::left_subtree)
    }

    fn right_subtree(&self) -> Option<&Self> {
        self.root().map(BinaryTreeNode::right_subtree)
    }

    fn subtree(&self, side: Side) -> Option<&Self> {
        self.root().map(|root| root.subtree(side))
    }

    fn subtrees(&self) -> Option<(&Self, &Self)> {
        self.root().map(BinaryTreeNode::subtrees)
    }
}

pub trait BinaryTreeNode {
    type Tree: BinaryTree<Node = Self>;

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

macro_rules! fmt_binary_tree {
    ($fn_name: ident, $fmt_trait: ident) => {
        pub fn $fn_name<T>(tree: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
        where
            T: BinaryTree,
            T::Node: $fmt_trait + BinaryTreeNode<Tree = T>,
        {
            fn recursive_fmt<T>(tree: &T, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
            where
                T: BinaryTree,
                T::Node: $fmt_trait + BinaryTreeNode<Tree = T>,
            {
                write!(f, "{prefix}")?;
                if is_left {
                    write!(f, "├──")?;
                } else {
                    write!(f, "└──")?;
                };
                if let Some(root) = tree.root() {
                    root.fmt(f)?;
                    writeln!(f, "")?;
                    let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                    recursive_fmt(root.left_subtree(), f, &new_prefix, true)?;
                    recursive_fmt(root.right_subtree(), f, &new_prefix, false)?;
                    Ok(())
                } else {
                    write!(f, "L\n")
                }
            }
            
            write!(f, "\n")?;
            recursive_fmt(tree, f, "", false)
        }
    };
}

fmt_binary_tree!(debug_binary_tree, Debug);
fmt_binary_tree!(display_binary_tree, Display);
