use lending_iterator::prelude::*;
use paste::paste;

use crate::binary_trees::{binary_tree, red_black_tree::{RedBlackNode, RedBlackTree}};

macro_rules! impl_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V> RedBlackTree<K, V> {
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

            pub struct [<$iter:camel>]<'t, K, V>(binary_tree::[<$iter:camel>]<'t, RedBlackNode<K, V>>);
            pub struct [<$iter:camel Mut>]<'t, K, V>(binary_tree::[<$iter:camel Mut>]<'t, RedBlackNode<K, V>>);
            pub struct [<Into $iter:camel>]<K, V>(binary_tree::[<Into $iter:camel>]<RedBlackNode<K, V>>);

            impl<'t, K, V> Iterator for [<$iter:camel>]<'t, K, V> {
                type Item = &'t RedBlackNode<K, V>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }

            #[gat]
            impl<'t, K, V> LendingIterator for [<$iter:camel Mut>]<'t, K, V> {
                type Item<'next>
                where 
                    Self: 'next,
                    = &'next mut RedBlackNode<K, V>;

                fn next(self: &mut [<$iter:camel Mut>]<'t, K, V>) -> Option<&mut RedBlackNode<K, V>> {
                    self.0.next()
                }
            }

            impl<K, V> Iterator for [<Into $iter:camel>]<K, V> {
                type Item = (K, V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(Into::into)
                }
            }
        }
    };
}

macro_rules! impl_iter_filtered {
    ($iter: ident) => {
        paste! {
            impl<K, V> RedBlackTree<K, V> {
                pub fn [<$iter:snake _filtered>]<P>(&self, subtree_filter: P) -> [<$iter:camel Filtered>]<'_, K, V, P>
                where
                    P: Fn(&RedBlackNode<K, V>) -> bool,
                {
                    [<$iter:camel Filtered>](self.0.[<$iter:snake _filtered>](subtree_filter))
                }

                pub fn [<$iter:snake _filtered_mut>]<P>(&mut self, subtree_filter: P) -> [<$iter:camel FilteredMut>]<'_, K, V, P>
                where
                    P: Fn(&RedBlackNode<K, V>) -> bool,
                {
                    [<$iter:camel FilteredMut>](self.0.[<$iter:snake _filtered_mut>](subtree_filter))
                }

                pub fn [<into_ $iter:snake _filtered>]<P>(self, subtree_filter: P) -> [<Into $iter:camel Filtered>]<K, V, P>
                where
                    P: Fn(&RedBlackNode<K, V>) -> bool,
                {
                    [<Into $iter:camel Filtered>](self.0.[<into_ $iter:snake _filtered>](subtree_filter))
                }
            }

            pub struct [<$iter:camel Filtered>]<'t, K, V, P>(binary_tree::[<$iter:camel Filtered>]<'t, RedBlackNode<K, V>, P>);
            pub struct [<$iter:camel FilteredMut>]<'t, K, V, P>(binary_tree::[<$iter:camel FilteredMut>]<'t, RedBlackNode<K, V>, P>);
            pub struct [<Into $iter:camel Filtered>]<K, V, P>(binary_tree::[<Into $iter:camel Filtered>]<RedBlackNode<K, V>, P>);

            impl<'t, K, V, P> Iterator for [<$iter:camel Filtered>]<'t, K, V, P>
            where
                P: Fn(&RedBlackNode<K, V>) -> bool,
            {
                type Item = &'t RedBlackNode<K, V>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }

            #[gat]
            impl<'t, K, V, P> LendingIterator for [<$iter:camel FilteredMut>]<'t, K, V, P>
            where
                P: Fn(&RedBlackNode<K, V>) -> bool,
            {
                type Item<'next>
                where 
                    Self: 'next,
                    = &'next mut RedBlackNode<K, V>;

                fn next(self: &mut [<$iter:camel FilteredMut>]<'t, K, V, P>) -> Option<&mut RedBlackNode<K, V>> {
                    self.0.next()
                }
            }

            impl<K, V, P> Iterator for [<Into $iter:camel Filtered>]<K, V, P>
            where
                P: Fn(&RedBlackNode<K, V>) -> bool,
            {
                type Item = (K, V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(Into::into)
                }
            }
        }
    };
}

impl_iter!(InorderIter);
impl_iter_filtered!(InorderIter);
impl_iter!(PreorderIter);
impl_iter_filtered!(PreorderIter);
impl_iter!(PostorderIter);
impl_iter_filtered!(PostorderIter);
