macro_rules! impl_iter {
    (
        $vis: vis struct $name: ident $(<$($gen: tt),*>)? ($inner: ty),
        $item: ty,
        $map: expr
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
    };
}

pub(crate) use impl_iter;
