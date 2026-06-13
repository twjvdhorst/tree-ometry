use paste::paste;

use super::{RbNode, RedBlackTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::impl_iter,
    binary_tree,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<T> RedBlackTree<T> {
                pub fn [<$iter:snake _iter>](&self) -> [<$iter:camel Iter>]<'_, T> {
                    [<$iter:camel Iter>](self.0.[<$iter:snake _iter>]())
                }

                pub fn [<$iter:snake _iter_mut>](&mut self) -> [<$iter:camel IterMut>]<'_, T> {
                    [<$iter:camel IterMut>](self.0.[<$iter:snake _iter_mut>]())
                }

                pub fn [<into_ $iter:snake _iter>](self) -> [<Into $iter:camel Iter>]<T> {
                    [<Into $iter:camel Iter>](self.0.[<into_ $iter:snake _iter>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel Iter>]<'t, T>(binary_tree::[<$iter:camel Iter>]<'t, RbNode<T>>),
                &'t T,
                RbNode::data
            );
            impl_iter!(
                pub struct [<$iter:camel IterMut>]<'t, T>(binary_tree::[<$iter:camel IterMut>]<'t, RbNode<T>>),
                &'t mut T,
                RbNode::data_mut
            );
            impl_iter!(
                pub struct [<Into $iter:camel Iter>]<T>(binary_tree::[<Into $iter:camel Iter>]<RbNode<T>>),
                T,
                RbNode::into_data
            );
        }
    };
}

// CursorMut does not have drain_subtree functions, as those will violate the red-black property of the tree.
macro_rules! impl_subtree_iter {
    ($iter: ident) => {
        paste! {
            impl<'t, T> Cursor<'t, T> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, T> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
            }

            impl<'t, T> CursorMut<'t, T> {
                pub fn [<$iter:snake _subtree_iter>](self) -> [<$iter:camel SubtreeIter>]<'t, T> {
                    [<$iter:camel SubtreeIter>](self.into_inner().[<$iter:snake _subtree_iter>]())
                }
                
                pub fn [<$iter:snake _subtree_iter_mut>](self) -> [<$iter:camel SubtreeIterMut>]<'t, T> {
                    [<$iter:camel SubtreeIterMut>](self.into_inner().[<$iter:snake _subtree_iter_mut>]())
                }
            }

            impl_iter!(
                pub struct [<$iter:camel SubtreeIter>]<'t, T>(binary_tree::[<$iter:camel SubtreeIter>]<'t, RbNode<T>>),
                &'t T,
                RbNode::data
            );
            impl_iter!(
                pub struct [<$iter:camel SubtreeIterMut>]<'t, T>(binary_tree::[<$iter:camel SubtreeIterMut>]<'t, RbNode<T>>),
                &'t mut T,
                RbNode::data_mut
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
