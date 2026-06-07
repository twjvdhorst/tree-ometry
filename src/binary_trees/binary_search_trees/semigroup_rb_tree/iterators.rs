use paste::paste;

use super::{SemigroupRbNode, SemigroupRbTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::impl_iter,
    binary_tree,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V, S> SemigroupRbTree<K, V, S> {
                pub fn [<$iter:snake _iter>](&self) -> [<$iter:camel Iter>]<'_, K, V, S> {
                    [<$iter:camel Iter>](self.0.[<$iter:snake _iter>]())
                }

                pub fn [<$iter:snake _iter_mut>](&mut self) -> [<$iter:camel IterMut>]<'_, K, V, S> {
                    [<$iter:camel IterMut>](self.0.[<$iter:snake _iter_mut>]())
                }

                pub fn [<into_ $iter:snake _iter>](self) -> [<Into $iter:camel Iter>]<K, V, S> {
                    [<Into $iter:camel Iter>](self.0.[<into_ $iter:snake _iter>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel Iter>]<'t, K, V, S>(binary_tree::[<$iter:camel Iter>]<'t, SemigroupRbNode<K, V, S>>),
                (&'t K, &'t V, &'t S),
                SemigroupRbNode::data
            );
            impl_iter!(
                pub struct [<$iter:camel IterMut>]<'t, K, V, S>(binary_tree::[<$iter:camel IterMut>]<'t, SemigroupRbNode<K, V, S>>),
                (&'t K, &'t mut V, &'t S),
                SemigroupRbNode::data_with_mut_value
            );
            impl_iter!(
                pub struct [<Into $iter:camel Iter>]<K, V, S>(binary_tree::[<Into $iter:camel Iter>]<SemigroupRbNode<K, V, S>>),
                (K, V, S),
                SemigroupRbNode::into_data
            );
        }
    };
}

// CursorMut does not have drain_subtree functions, as those will violate the red-black property of the tree.
macro_rules! impl_subtree_iter {
    ($iter: ident) => {
        paste! {
            impl<'t, K, V, S> Cursor<'t, K, V, S> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, K, V, S> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
            }

            impl<'t, K, V, S> CursorMut<'t, K, V, S> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, K, V, S> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
                
                pub fn [<$iter:snake _subtree_iter_mut>](self) -> [<$iter:camel SubtreeIterMut>]<'t, K, V, S> {
                    [<$iter:camel SubtreeIterMut>](self.into_inner().[<$iter:snake _subtree_iter_mut>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel SubtreeIter>]<'t, K, V, S>(binary_tree::[<$iter:camel SubtreeIter>]<'t, SemigroupRbNode<K, V, S>>),
                (&'t K, &'t V, &'t S),
                SemigroupRbNode::data
            );
            impl_iter!(
                pub struct [<$iter:camel SubtreeIterMut>]<'t, K, V, S>(binary_tree::[<$iter:camel SubtreeIterMut>]<'t, SemigroupRbNode<K, V, S>>),
                (&'t K, &'t mut V, &'t S),
                SemigroupRbNode::data_with_mut_value
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
