mod traits;

pub mod red_black_tree;
pub mod semigroup_rb_tree;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
enum Color {
    Red,
    Black,
}
