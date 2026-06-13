use paste::paste;

use super::{MinMaxRbData, MinMaxRbTree, Cursor, CursorMut};
use crate::binary_trees::{
    impl_iterator_macro::impl_iter,
    binary_search_trees::red_black_trees::base,
};

macro_rules! impl_tree_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V> MinMaxRbTree<K, V> {
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
                pub struct [<$iter:camel Iter>]<'t, K, V>(base::[<$iter:camel Iter>]<'t, MinMaxRbData<K, V>>),
                (&'t K, &'t V),
                MinMaxRbData::data
            );
            impl_iter!(
                pub struct [<$iter:camel IterMut>]<'t, K, V>(base::[<$iter:camel IterMut>]<'t, MinMaxRbData<K, V>>),
                (&'t K, &'t mut V),
                MinMaxRbData::data_with_mut_value
            );
            impl_iter!(
                pub struct [<Into $iter:camel Iter>]<K, V>(base::[<Into $iter:camel Iter>]<MinMaxRbData<K, V>>),
                (K, V),
                MinMaxRbData::into_data
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
                pub struct [<$iter:camel SubtreeIter>]<'t, K, V>(base::[<$iter:camel SubtreeIter>]<'t, MinMaxRbData<K, V>>),
                (&'t K, &'t V),
                MinMaxRbData::data
            );
            impl_iter!(
                pub struct [<$iter:camel SubtreeIterMut>]<'t, K, V>(base::[<$iter:camel SubtreeIterMut>]<'t, MinMaxRbData<K, V>>),
                (&'t K, &'t mut V),
                MinMaxRbData::data_with_mut_value
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
