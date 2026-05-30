use std::fmt::Display;

pub mod binary_tree;
pub mod binary_tree_cursor;
pub mod cursor_errors;

//pub mod binary_search_trees;
//pub mod heaps;

pub struct Neighborhood<T> {
    pub node: Option<T>,
    pub parent: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Neighborhood<T> {
    pub fn map<U, F>(self, mut f: F) -> Neighborhood<U>
    where 
        F: FnMut(T) -> U,
    {
        let Self { node, parent, left, right } = self;
        Neighborhood {
            node: node.map(&mut f),
            parent: parent.map(&mut f),
            left: left.map(&mut f),
            right: right.map(&mut f),
        }
    }
}

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

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Left => write!(f, "Left"),
            Side::Right => write!(f, "Right"),
        }
    }
}
