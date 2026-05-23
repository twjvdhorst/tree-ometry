use paste::paste;

use super::{BstNode, BinarySearchTree};
use crate::binary_trees::binary_tree;

macro_rules! impl_iter {
    ($iter: ident) => {
        paste! {
            impl<K, V> BinarySearchTree<K, V> {
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

            pub struct [<$iter:camel>]<'t, K, V>(binary_tree::[<$iter:camel>]<'t, BstNode<K, V>>);
            pub struct [<$iter:camel Mut>]<'t, K, V>(binary_tree::[<$iter:camel Mut>]<'t, BstNode<K, V>>);
            pub struct [<Into $iter:camel>]<K, V>(binary_tree::[<Into $iter:camel>]<BstNode<K, V>>);

            impl<'t, K, V> Iterator for [<$iter:camel>]<'t, K, V> {
                type Item = (&'t K, &'t V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(BstNode::data)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V> Iterator for [<$iter:camel Mut>]<'t, K, V> {
                type Item = (&'t K, &'t mut V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(BstNode::data_with_mut_value)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<K, V> Iterator for [<Into $iter:camel>]<K, V> {
                type Item = (K, V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(Into::into)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }
        }
    };
}

macro_rules! impl_iter_filtered {
    ($iter: ident) => {
        paste! {
            impl<K, V> BinarySearchTree<K, V> {
                pub fn [<$iter:snake _filtered>]<P>(&self, subtree_filter: P) -> [<$iter:camel Filtered>]<'_, K, V, P>
                where
                    P: Fn(&BstNode<K, V>) -> bool,
                {
                    [<$iter:camel Filtered>](self.0.[<$iter:snake _filtered>](subtree_filter))
                }

                pub fn [<$iter:snake _filtered_mut>]<P>(&mut self, subtree_filter: P) -> [<$iter:camel FilteredMut>]<'_, K, V, P>
                where
                    P: Fn(&BstNode<K, V>) -> bool,
                {
                    [<$iter:camel FilteredMut>](self.0.[<$iter:snake _filtered_mut>](subtree_filter))
                }

                pub fn [<into_ $iter:snake _filtered>]<P>(self, subtree_filter: P) -> [<Into $iter:camel Filtered>]<K, V, P>
                where
                    P: Fn(&BstNode<K, V>) -> bool,
                {
                    [<Into $iter:camel Filtered>](self.0.[<into_ $iter:snake _filtered>](subtree_filter))
                }
            }

            pub struct [<$iter:camel Filtered>]<'t, K, V, P>(binary_tree::[<$iter:camel Filtered>]<'t, BstNode<K, V>, P>);
            pub struct [<$iter:camel FilteredMut>]<'t, K, V, P>(binary_tree::[<$iter:camel FilteredMut>]<'t, BstNode<K, V>, P>);
            pub struct [<Into $iter:camel Filtered>]<K, V, P>(binary_tree::[<Into $iter:camel Filtered>]<BstNode<K, V>, P>);

            impl<'t, K, V, P> Iterator for [<$iter:camel Filtered>]<'t, K, V, P>
            where
                P: Fn(&BstNode<K, V>) -> bool,
            {
                type Item = (&'t K, &'t V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(BstNode::data)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V, P> Iterator for [<$iter:camel FilteredMut>]<'t, K, V, P>
            where
                P: Fn(&BstNode<K, V>) -> bool,
            {
                type Item = (&'t K, &'t mut V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(BstNode::data_with_mut_value)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<K, V, P> Iterator for [<Into $iter:camel Filtered>]<K, V, P>
            where
                P: Fn(&BstNode<K, V>) -> bool,
            {
                type Item = (K, V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(Into::into)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
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
