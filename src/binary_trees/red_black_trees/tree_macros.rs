use paste::paste;

/// Iterators.
macro_rules! make_iter {
    ($vis: vis, $iter_name: ident, $iter_type: ident) => {
        paste!{
            $vis fn $iter_name(&'_ self) -> $iter_type<'_, Self, impl Fn(&Self) -> bool> {
                self.[<$iter_name _filtered>](|_| true)
            }

            $vis fn [<$iter_name _filtered>]<F>(&'_ self, f: F) -> $iter_type<'_, Self, F>
            where
                F: Fn(&Self) -> bool,
            {
                $iter_type::new(self, f)
            }
        }
    };
}

macro_rules! make_iter_mut {
    ($vis: vis, $iter_name: ident, $iter_type: ident) => {
        paste!{
            $vis fn $iter_name(&'_ mut self) -> $iter_type<'_, Self, impl Fn(&Self) -> bool> {
                self.[<$iter_name _filtered>](|_| true)
            }

            $vis fn [<$iter_name _filtered>]<F>(&'_ mut self, f: F) -> $iter_type<'_, Self, F>
            where
                F: Fn(&Self) -> bool,
            {
                $iter_type::new(self, f)
            }
        }
    };
}

pub(crate) use make_iter;
pub(crate) use make_iter_mut;
