use crate::binary_trees::{
    binary_tree::BinaryTree, 
    red_black_tree::RedBlackNode,
    semigroup_rb_tree::{
        SemigroupRbNode, 
        SemigroupRbTree,
    },
};

use ref_cast::RefCast;

impl<K, V> AsRef<RedBlackNode<K, V>> for SemigroupRbNode<K, V, ()> {
    fn as_ref(&self) -> &RedBlackNode<K, V> {
        RedBlackNode::ref_cast(self)
    }
}

impl<K, V> AsMut<RedBlackNode<K, V>> for SemigroupRbNode<K, V, ()> {
    fn as_mut(&mut self) -> &mut RedBlackNode<K, V> {
        RedBlackNode::ref_cast_mut(self)
    }
}

impl<K, V, S> From<SemigroupRbTree<K, V, S>> for BinaryTree<(K, V)> {
    fn from(value: SemigroupRbTree<K, V, S>) -> Self {
        value.0.map(SemigroupRbNode::into_data)
    }
}
