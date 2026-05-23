use std::fmt::Display;

pub mod binary_tree;
pub mod binary_tree_cursor;
pub mod cursor_errors;


pub mod binary_search_tree;
pub mod cartesian_tree;
/*pub mod red_black_tree;
pub mod semigroup_rb_tree;
*/

pub struct Neighborhood<T> {
    pub node: Option<T>,
    pub parent: Option<T>,
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T> Neighborhood<T> {
    pub fn map<U, F>(self, f: F) -> Neighborhood<U>
    where 
        F: Fn(T) -> U,
    {
        let Self { node, parent, left, right } = self;
        Neighborhood {
            node: node.map(&f),
            parent: parent.map(&f),
            left: left.map(&f),
            right: right.map(&f),
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
