use paste::paste;

use super::{CartesianNode, CartesianTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::impl_iter,
    binary_tree,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V, C> CartesianTree<K, V, C> {
                pub fn [<$iter:snake>](&self) -> [<$iter:camel>]<'_, K, V> {
                    [<$iter:camel>](self.0.[<$iter:snake>]())
                }

                pub fn [<$iter:snake _mut>](&mut self) -> [<$iter:camel Mut>]<'_, K, V> {
                    [<$iter:camel Mut>](self.0.[<$iter:snake _mut>]())
                }

                pub fn [<into_ $iter:snake>](self) -> [<Into $iter:camel>]<K, V> {
                    [<Into $iter:camel>](self.0.[<into_ $iter:snake>]())
                }
            }

            pub struct [<$iter:camel>]<'t, K, V>(binary_tree::[<$iter:camel>]<'t, CartesianNode<K, V>>);
            pub struct [<$iter:camel Mut>]<'t, K, V>(binary_tree::[<$iter:camel Mut>]<'t, CartesianNode<K, V>>);
            pub struct [<Into $iter:camel>]<K, V>(binary_tree::[<Into $iter:camel>]<CartesianNode<K, V>>);

            impl_iter!([<$iter:camel>]<'t, K, V>, (&'t K, &'t V), CartesianNode::data);
            impl_iter!([<$iter:camel Mut>]<'t, K, V>, (&'t K, &'t mut V), CartesianNode::data_with_mut_value);
            impl_iter!([<Into $iter:camel>]<K, V>, (K, V), CartesianNode::into_data);
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
                pub fn [<$iter:snake _subtree_iter_mut>](self) -> [<$iter:camel SubtreeIterMut>]<'t, K, V> {
                    [<$iter:camel SubtreeIterMut>](self.into_inner().[<$iter:snake _subtree_iter_mut>]())
                }

                pub fn [<drain_subtree_ $iter:snake>](self) -> [<DrainSubtree $iter:camel>]<'t, K, V> {
                    [<DrainSubtree $iter:camel>](self.into_inner().[<drain_subtree_ $iter:snake>]())
                }
            }

            pub struct [<$iter:camel SubtreeIter>]<'t, K, V>(binary_tree::[<$iter:camel SubtreeIter>]<'t, CartesianNode<K, V>>);
            pub struct [<$iter:camel SubtreeIterMut>]<'t, K, V>(binary_tree::[<$iter:camel SubtreeIterMut>]<'t, CartesianNode<K, V>>);
            pub struct [<DrainSubtree $iter:camel>]<'t, K, V>(binary_tree::[<DrainSubtree $iter:camel>]<'t, CartesianNode<K, V>>);

            impl_iter!([<$iter:camel SubtreeIter>]<'t, K, V>, (&'t K, &'t V), CartesianNode::data);
            impl_iter!([<$iter:camel SubtreeIterMut>]<'t, K, V>, (&'t K, &'t mut V), CartesianNode::data_with_mut_value);
            impl_iter!([<DrainSubtree $iter:camel>]<'t, K, V>, (K, V), CartesianNode::into_data);
        }
    };
}

impl_tree_iter!(InorderIter);
impl_tree_iter!(PreorderIter);
impl_tree_iter!(PostorderIter);

impl_subtree_iter!(Inorder);
impl_subtree_iter!(Preorder);
impl_subtree_iter!(Postorder);
