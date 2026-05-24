use paste::paste;

use super::{RedBlackNode, RedBlackTree};
use crate::binary_trees::{
    tree_iterators::TreeIterator,
    binary_tree,
};

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
                type Item = (&'t K, &'t V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(RedBlackNode::data)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V> TreeIterator<RedBlackNode<K, V>> for [<$iter:camel>]<'t, K, V> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&RedBlackNode<K, V>) -> bool
                {
                    self.0.next_with_subtree_filter(predicate).map(RedBlackNode::data)
                }
            }

            impl<'t, K, V> Iterator for [<$iter:camel Mut>]<'t, K, V> {
                type Item = (&'t K, &'t mut V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(RedBlackNode::data_with_mut_value)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V> TreeIterator<RedBlackNode<K, V>> for [<$iter:camel Mut>]<'t, K, V> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&RedBlackNode<K, V>) -> bool
                {
                    self.0.next_with_subtree_filter(predicate).map(RedBlackNode::data_with_mut_value)
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

            impl<K, V> TreeIterator<RedBlackNode<K, V>> for [<Into $iter:camel>]<K, V> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&RedBlackNode<K, V>) -> bool
                {
                    self.0.next_with_subtree_filter(predicate).map(Into::into)
                }
            }
        }
    };
}

impl_iter!(InorderIter);
impl_iter!(PreorderIter);
impl_iter!(PostorderIter);
