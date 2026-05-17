use crate::binary_trees::{binary_tree::BinaryTree, red_black_tree::RedBlackTree, semigroup_rb_tree::SemigroupRbTree};

impl<K, V> From<SemigroupRbTree<K, V, ()>> for RedBlackTree<K, V> {
    fn from(value: SemigroupRbTree<K, V, ()>) -> Self {
        Self(value)
    }
}

impl<K, V> From<RedBlackTree<K, V>> for BinaryTree<(K, V)> {
    fn from(value: RedBlackTree<K, V>) -> Self {
        value.0.into()
    }
}
