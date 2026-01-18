pub mod red_black_tree;
pub mod tree_errors;
pub mod tree_iterators;
pub mod tree_traits;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}
