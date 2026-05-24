use paste::paste;

use super::{SemigroupRbNode, SemigroupRbTree};
use crate::binary_trees::{
    tree_iterators::TreeIterator,
    binary_tree,
};

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
                type Item = (&'t K, &'t V, &'t S);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(SemigroupRbNode::data)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V, S> TreeIterator<SemigroupRbNode<K, V, S>> for [<$iter:camel>]<'t, K, V, S> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&SemigroupRbNode<K, V, S>) -> bool
                {
                    self.0.next_with_subtree_filter(predicate).map(SemigroupRbNode::data)
                }
            }

            impl<'t, K, V, S> Iterator for [<$iter:camel Mut>]<'t, K, V, S> {
                type Item = (&'t K, &'t mut V, &'t S);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(SemigroupRbNode::data_with_mut_value)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<'t, K, V, S> TreeIterator<SemigroupRbNode<K, V, S>> for [<$iter:camel Mut>]<'t, K, V, S> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&SemigroupRbNode<K, V, S>) -> bool
                {
                    self.0.next_with_subtree_filter(predicate).map(SemigroupRbNode::data_with_mut_value)
                }
            }

            impl<K, V, S> Iterator for [<Into $iter:camel>]<K, V, S> {
                type Item = (K, V);

                fn next(&mut self) -> Option<Self::Item> {
                    self.0.next().map(Into::into)
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    self.0.size_hint()
                }
            }

            impl<K, V, S> TreeIterator<SemigroupRbNode<K, V, S>> for [<Into $iter:camel>]<K, V, S> {
                fn next_with_subtree_filter<P>(&mut self, predicate: P) -> Option<Self::Item>
                where 
                    P: FnMut(&SemigroupRbNode<K, V, S>) -> bool
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
