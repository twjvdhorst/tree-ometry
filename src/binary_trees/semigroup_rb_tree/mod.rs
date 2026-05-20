mod semigroup_rb_tree;
mod tree_semigroup;
mod cursors;
mod iterators;
#[cfg(feature = "serde")]
pub mod serialization;

pub use semigroup_rb_tree::*;
pub use tree_semigroup::*;
pub use cursors::*;
pub use iterators::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(super) enum Color {
    Red,
    Black,
}
