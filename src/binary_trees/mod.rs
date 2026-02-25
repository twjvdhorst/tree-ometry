pub mod tree_iterators;
pub mod red_black_trees;
pub mod traits;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn opposite(&self) -> Side {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}
