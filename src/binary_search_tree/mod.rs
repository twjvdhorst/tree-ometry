pub mod red_black_node;
pub mod red_black_tree;
//pub mod tree_iterators;
pub mod tree_traits;

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
