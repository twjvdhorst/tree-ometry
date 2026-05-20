use lending_iterator::prelude::*;
use paste::paste;

use crate::binary_trees::{binary_tree, semigroup_rb_tree::{SemigroupRbNode, SemigroupRbTree}};

macro_rules! impl_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V, S> SemigroupRbTree<K, V, S> {
                pub fn [<$iter:snake>](&self) -> [<$iter:camel>]<'_, K, V, S> {
                    [<$iter:camel>](self.0.[<$iter:snake>]())
                }

                pub fn [<$iter:snake _mut>](&mut self) -> [<$iter:camel Mut>]<'_, K, V, S> {
                    [<$iter:camel Mut>](self.0.[<$iter:snake _mut>]())
                }

                pub fn [<into_ $iter:snake>](self) -> [<Into $iter:camel>]<K, V, S> {
                    [<Into $iter:camel>](self.0.[<into_ $iter:snake>]())
                }
            }

            pub struct [<$iter:camel>]<'t, K, V, S>(binary_tree::[<$iter:camel>]<'t, SemigroupRbNode<K, V, S>>);
            pub struct [<$iter:camel Mut>]<'t, K, V, S>(binary_tree::[<$iter:camel Mut>]<'t, SemigroupRbNode<K, V, S>>);
            pub struct [<Into $iter:camel>]<K, V, S>(binary_tree::[<Into $iter:camel>]<SemigroupRbNode<K, V, S>>);

            impl<'t, K, V, S> Iterator for [<$iter:camel>]<'t, K, V, S> {
                type Item = &'t SemigroupRbNode<K, V, S>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }

            #[gat]
            impl<'t, K, V, S> LendingIterator for [<$iter:camel Mut>]<'t, K, V, S> {
                type Item<'next>
                where 
                    Self: 'next,
                    = &'next mut SemigroupRbNode<K, V, S>;

                fn next(self: &mut [<$iter:camel Mut>]<'t, K, V, S>) -> Option<&mut SemigroupRbNode<K, V, S>> {
                    self.0.next()
                }
            }

            impl<K, V, S> Iterator for [<Into $iter:camel>]<K, V, S> {
                type Item = SemigroupRbNode<K, V, S>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }
        }
    };
}

macro_rules! impl_iter_filtered {
    ($iter: ident) => {
        paste! {
            impl<K, V, S> SemigroupRbTree<K, V, S> {
                pub fn [<$iter:snake _filtered>]<P>(&self, subtree_filter: P) -> [<$iter:camel Filtered>]<'_, K, V, S, P>
                where
                    P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
                {
                    [<$iter:camel Filtered>](self.0.[<$iter:snake _filtered>](subtree_filter))
                }

                pub fn [<$iter:snake _filtered_mut>]<P>(&mut self, subtree_filter: P) -> [<$iter:camel FilteredMut>]<'_, K, V, S, P>
                where
                    P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
                {
                    [<$iter:camel FilteredMut>](self.0.[<$iter:snake _filtered_mut>](subtree_filter))
                }

                pub fn [<into_ $iter:snake _filtered>]<P>(self, subtree_filter: P) -> [<Into $iter:camel Filtered>]<K, V, S, P>
                where
                    P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
                {
                    [<Into $iter:camel Filtered>](self.0.[<into_ $iter:snake _filtered>](subtree_filter))
                }
            }

            pub struct [<$iter:camel Filtered>]<'t, K, V, S, P>(binary_tree::[<$iter:camel Filtered>]<'t, SemigroupRbNode<K, V, S>, P>);
            pub struct [<$iter:camel FilteredMut>]<'t, K, V, S, P>(binary_tree::[<$iter:camel FilteredMut>]<'t, SemigroupRbNode<K, V, S>, P>);
            pub struct [<Into $iter:camel Filtered>]<K, V, S, P>(binary_tree::[<Into $iter:camel Filtered>]<SemigroupRbNode<K, V, S>, P>);

            impl<'t, K, V, S, P> Iterator for [<$iter:camel Filtered>]<'t, K, V, S, P>
            where
                P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
            {
                type Item = &'t SemigroupRbNode<K, V, S>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
                }
            }

            #[gat]
            impl<'t, K, V, S, P> LendingIterator for [<$iter:camel FilteredMut>]<'t, K, V, S, P>
            where
                P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
            {
                type Item<'next>
                where 
                    Self: 'next,
                    = &'next mut SemigroupRbNode<K, V, S>;

                fn next(self: &mut [<$iter:camel FilteredMut>]<'t, K, V, S, P>) -> Option<&mut SemigroupRbNode<K, V, S>> {
                    self.0.next()
                }
            }

            impl<K, V, S, P> Iterator for [<Into $iter:camel Filtered>]<K, V, S, P>
            where
                P: Fn(&SemigroupRbNode<K, V, S>) -> bool,
            {
                type Item = SemigroupRbNode<K, V, S>;

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next()
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
