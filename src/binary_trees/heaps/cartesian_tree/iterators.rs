use paste::paste;

use super::{CartesianNode, CartesianTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::{conditionally_expand, impl_iter},
    binary_tree,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V, C> CartesianTree<K, V, C> {
                pub fn [<$iter:snake _iter>](&self) -> [<$iter:camel Iter>]<'_, K, V> {
                    [<$iter:camel Iter>](self.0.[<$iter:snake _iter>]())
                }

                pub fn [<$iter:snake _iter_mut>](&mut self) -> [<$iter:camel IterMut>]<'_, K, V> {
                    [<$iter:camel IterMut>](self.0.[<$iter:snake _iter_mut>]())
                }

                pub fn [<into_ $iter:snake _iter>](self) -> [<Into $iter:camel Iter>]<K, V> {
                    [<Into $iter:camel Iter>](self.0.[<into_ $iter:snake _iter>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel Iter>]<'t, K, V>(binary_tree::[<$iter:camel Iter>]<'t, CartesianNode<K, V>>),
                (&'t K, &'t V),
                CartesianNode::data,
                true,
                true,
            );
            impl_iter!(
                pub struct [<$iter:camel IterMut>]<'t, K, V>(binary_tree::[<$iter:camel IterMut>]<'t, CartesianNode<K, V>>),
                (&'t K, &'t mut V),
                CartesianNode::data_with_mut_value,
                true,
                true,
            );
            impl_iter!(
                pub struct [<Into $iter:camel Iter>]<K, V>(binary_tree::[<Into $iter:camel Iter>]<CartesianNode<K, V>>),
                (K, V),
                CartesianNode::into_data,
                true,
                true,
            );
        }
    };
}

macro_rules! impl_subtree_iter {
    ($iter: ident) => {
        paste! {
            impl<'t, K, V> Cursor<'t, K, V> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, K, V> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
            }

            impl<'t, K, V> CursorMut<'t, K, V> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, K, V> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
                
                pub fn [<$iter:snake _subtree_iter_mut>](self) -> [<$iter:camel SubtreeIterMut>]<'t, K, V> {
                    [<$iter:camel SubtreeIterMut>](self.into_inner().[<$iter:snake _subtree_iter_mut>]())
                }

                pub fn [<drain_subtree_ $iter:snake>](self) -> [<DrainSubtree $iter:camel>]<'t, K, V> {
                    [<DrainSubtree $iter:camel>](self.into_inner().[<drain_subtree_ $iter:snake>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel SubtreeIter>]<'t, K, V>(binary_tree::[<$iter:camel SubtreeIter>]<'t, CartesianNode<K, V>>),
                (&'t K, &'t V),
                CartesianNode::data,
                true,
                false,
            );
            impl_iter!(
                pub struct [<$iter:camel SubtreeIterMut>]<'t, K, V>(binary_tree::[<$iter:camel SubtreeIterMut>]<'t, CartesianNode<K, V>>),
                (&'t K, &'t mut V),
                CartesianNode::data_with_mut_value,
                true,
                false,
            );
            impl_iter!(
                pub struct [<DrainSubtree $iter:camel>]<'t, K, V>(binary_tree::[<DrainSubtree $iter:camel>]<'t, CartesianNode<K, V>>),
                (K, V),
                CartesianNode::into_data,
                true,
                false,
            );
        }
    };
}

impl_tree_iter!(Inorder);
impl_tree_iter!(Preorder);
impl_tree_iter!(Postorder);

impl_subtree_iter!(Inorder);
impl_subtree_iter!(Preorder);
impl_subtree_iter!(Postorder);
