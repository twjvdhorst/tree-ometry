macro_rules! conditionally_expand {
    (
        true,
        $fragment: item
    ) => {
        $fragment
    };
    (
        false,
        $fragment: item
    ) => {
    };
}

macro_rules! impl_iter {
    (
        $vis: vis struct $name: ident $(<$($gen: tt),*>)? ($inner: ty),
        $item: ty,
        $map: expr,
        $fused: tt,
        $exact_sized: tt$(,)?
    ) => {
        $vis struct $name $(<$($gen),*>)? ($inner);

        impl$(<$($gen),*>)? Iterator for $name$(<$($gen),*>)? {
            type Item = $item;

            fn next(&mut self) -> Option<Self::Item> {
                self.0.next().map($map)
            }

            fn size_hint(&self) -> (usize, Option<usize>) {
                self.0.size_hint()
            }
        }

        conditionally_expand!(
            $fused,
            impl$(<$($gen),*>)? std::iter::FusedIterator for $name$(<$($gen),*>)? {}
        );
        conditionally_expand!(
            $exact_sized,
            impl$(<$($gen),*>)? ExactSizeIterator for $name$(<$($gen),*>)? {
                fn len(&self) -> usize {
                    self.0.len()
                }
            }
        );
    };
}

pub(crate) use conditionally_expand;
pub(crate) use impl_iter;
