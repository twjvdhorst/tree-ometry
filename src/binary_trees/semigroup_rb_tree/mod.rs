mod semigroup_rb_tree;
mod tree_semigroup;
mod cursors;

pub use semigroup_rb_tree::*;
pub use tree_semigroup::*;
pub use cursors::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Color {
    Red,
    Black,
}
