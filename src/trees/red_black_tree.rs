use super::binary_tree::BinaryTree;
use super::red_black_node::RedBlackNode;

pub struct RedBlackTree<T> {
    key: T,
    left: Option<Box<RedBlackNode<T, Self>>>,
    right: Option<Box<RedBlackNode<T, Self>>>,
}

impl<T> BinaryTree for RedBlackTree<T> {
    type Node = RedBlackNode<T, Self>;
    type Edge = Box<Self::Node>;
    
    fn get_left(&self) -> Option<&Self::Node> {
        self.left.as_ref().map(|left| left.as_ref())
    }
    
    fn get_right(&self) -> Option<&Self::Node> {
        self.right.as_ref().map(|right| right.as_ref())
    }
    
    fn get_left_mut(&mut self) -> Option<&mut Self::Node> {
        self.left.as_mut().map(|left| left.as_mut())
    }
    
    fn get_right_mut(&mut self) -> Option<&mut Self::Node> {
        self.right.as_mut().map(|right| right.as_mut())
    }
    
    fn has_left(&self) -> bool {
        self.left.is_some()
    }
    
    fn has_right(&self) -> bool {
        self.right.is_some()
    }
    
    fn attach_left(&mut self, tree: impl Into<Self::Edge>) -> bool {
        if !self.has_left() {
            self.left = Some(tree.into());
            true
        } else { false }
    }
    
    fn attach_right(&mut self, tree: impl Into<Self::Edge>) -> bool {
        if !self.has_right() {
            self.right = Some(tree.into());
            true
        } else { false }
    }
    
    fn detach_left(&mut self) -> Option<Self::Edge> {
        self.left.take()
    }
    
    fn detach_right(&mut self) -> Option<Self::Edge> {
        self.right.take()
    }
    
    fn replace_left(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> {
        self.left.replace(tree.into())
    }
    
    fn replace_right(&mut self, tree: impl Into<Self::Edge>) -> Option<Self::Edge> {
        self.right.replace(tree.into())
    }
}


#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use rand::prelude::*;

    use super::*;
    use crate::trees::red_black_node::Color;

    fn assert_binary_search_tree<T: Clone + Ord>(root: &RedBlackNode<T, RedBlackTree<T>>) {
        fn assert_binary_search_tree_recursive<T: Clone + Ord>(root: Option<&RedBlackNode<T, RedBlackTree<T>>>) -> Option<(T, T)> {
            let Some(root) = root else { return None; };
            if let Some(max_left) = assert_binary_search_tree_recursive(root.get_left()).map(|(_, max)| max) {
                assert_eq!(T::cmp(root.key(), &max_left), Ordering::Greater);
            }
            if let Some(min_right) = assert_binary_search_tree_recursive(root.get_right()).map(|(min, _)| min) {
                assert_eq!(T::cmp(root.key(), &min_right), Ordering::Less);
            }
            Some((
                assert_binary_search_tree_recursive(root.get_left()).map_or(root.key().clone(), |(min, _)| min),
                assert_binary_search_tree_recursive(root.get_right()).map_or(root.key().clone(), |(_, max)| max)
            ))
        }
        assert_binary_search_tree_recursive(Some(root));
    }

    /// Asserts the given tree is a valid red-black tree
    fn assert_valid_tree<T: Clone + Ord>(root: &RedBlackNode<T, RedBlackTree<T>>) {
        // Asserts the given tree is a valid red-black tree, and returns the number of black nodes on any root-to-leaf path in the tree
        fn assert_valid_tree_recursive<T: Clone + Ord>(root: Option<&RedBlackNode<T, RedBlackTree<T>>>) -> usize {
            // Leaves are considered black
            let Some(root) = root else { return 1; };

            // Assert no consecutive red nodes
            if root.color() == Color::Red {
                assert_ne!(root.get_left().map(|left| left.color()), Some(Color::Red));
                assert_ne!(root.get_right().map(|right| right.color()), Some(Color::Red));
            }

            // Assert validity of subtrees
            let num_black_left = assert_valid_tree_recursive(root.get_left());
            let num_black_right = assert_valid_tree_recursive(root.get_right());

            // Assert black counts match
            assert_eq!(num_black_left, num_black_right);

            // Return number of black nodes on any root-to-leaf path
            if root.color() == Color::Red {
                num_black_left
            } else {
                1 + num_black_left
            }
        }

        assert_eq!(root.color(), Color::Black);
        assert_binary_search_tree(root);
        assert_valid_tree_recursive(Some(root));
    }

    #[test]
    fn test_insertion() {
        // Test inserting values in order
        let mut tree = RedBlackNode::new(0);
        for key in 1..=30 {
            tree.insert(key);
        }
        assert_valid_tree(&tree);

        // Test inserting values in random order
        let mut rng = rand::rng();
        for _ in 0..5 {
            let mut tree = RedBlackNode::new(0);
            let mut keys = (1..=30).collect::<Vec<_>>();
            keys.shuffle(&mut rng);
            for key in keys {
                tree.insert(key);
            }
            assert_valid_tree(&tree);
        }
    }
}
