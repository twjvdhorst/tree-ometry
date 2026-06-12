use paste::paste;

use super::{RedBlackTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::impl_iter,
    binary_search_trees::semigroup_rb_tree,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V> RedBlackTree<K, V> {
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
                pub struct [<$iter:camel Iter>]<'t, K, V>(semigroup_rb_tree::[<$iter:camel Iter>]<'t, K, V, ()>),
                (&'t K, &'t V),
                |(k, v, _)| (k, v)
            );
            impl_iter!(
                pub struct [<$iter:camel IterMut>]<'t, K, V>(semigroup_rb_tree::[<$iter:camel IterMut>]<'t, K, V, ()>),
                (&'t K, &'t mut V),
                |(k, v, _)| (k, v)
            );
            impl_iter!(
                pub struct [<Into $iter:camel Iter>]<K, V>(semigroup_rb_tree::[<Into $iter:camel Iter>]<K, V, ()>),
                (K, V),
                |(k, v, _)| (k, v)
            );
        }
    };
}

// CursorMut does not have drain_subtree functions, as those will violate the red-black property of the tree.
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
            }

            impl_iter!(
                pub struct [<$iter:camel SubtreeIter>]<'t, K, V>(semigroup_rb_tree::[<$iter:camel SubtreeIter>]<'t, K, V, ()>),
                (&'t K, &'t V),
                |(k, v, _)| (k, v)
            );
            impl_iter!(
                pub struct [<$iter:camel SubtreeIterMut>]<'t, K, V>(semigroup_rb_tree::[<$iter:camel SubtreeIterMut>]<'t, K, V, ()>),
                (&'t K, &'t mut V),
                |(k, v, _)| (k, v)
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
