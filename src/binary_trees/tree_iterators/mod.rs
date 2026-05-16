mod preorder;
mod postorder;
mod inorder;

pub use preorder::*;
pub use postorder::*;
pub use inorder::*;

pub(crate) use impl_iters::impl_iters;
mod impl_iters {
    macro_rules! impl_iters {
        ($vis: vis, $iter: ident) => {
            paste! {
                $vis fn [<$iter:lower _iter>](&self) -> [<$iter:camel Iter>]<'_, Self> {
                    [<$iter:camel Iter>]::new(self)
                }

                $vis fn [<$iter:lower _iter_filtered>]<P>(&self, subtree_filter: P) -> [<$iter:camel IterFiltered>]<'_, Self, P>
                where 
                    P: Fn(&BinaryTreeNode<T>) -> bool,
                {
                    [<$iter:camel IterFiltered>]::new(self, subtree_filter)
                }

                $vis fn [<$iter:lower _iter_mut>](&mut self) -> [<$iter:camel IterMut>]<'_, Self> {
                    [<$iter:camel IterMut>]::new(self)
                }

                $vis fn [<$iter:lower _iter_filtered_mut>]<P>(&mut self, subtree_filter: P) -> [<$iter:camel IterFilteredMut>]<'_, Self, P>
                where 
                    P: Fn(&BinaryTreeNode<T>) -> bool,
                {
                    [<$iter:camel IterFilteredMut>]::new(self, subtree_filter)
                }
            }
        };
    }

    pub(crate) use impl_iters;
}
