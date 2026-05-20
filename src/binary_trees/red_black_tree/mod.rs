mod red_black_tree;
mod cursors;
mod iterators;
#[cfg(feature = "serde")]
mod serialization;

pub use red_black_tree::*;
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

