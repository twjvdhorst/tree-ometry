use std::fmt::{self, Debug, Display};

use crate::binary_trees::traits::binary_tree_cursor::{PeekingCursor, PeekingCursorMut};

pub trait BinaryTree {
    type Node;
    type Cursor<'c>: PeekingCursor<'c, Item = Self::Node>
    where Self: 'c;
    
    fn cursor(&self) -> Self::Cursor<'_>;
}

pub trait BinaryTreeMut: BinaryTree {
    type CursorMut<'c>: PeekingCursorMut<Item = Self::Node>
    where Self: 'c;

    fn cursor_mut(&mut self) -> Self::CursorMut<'_>;
}

macro_rules! fmt_binary_tree {
    ($fn_name: ident, $fmt_trait: ident) => {
        pub fn $fn_name<T>(tree: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result
        where
            T: BinaryTree,
            T::Node: $fmt_trait,
        {
            fn recursive_fmt<'t, C>(cursor: C, f: &mut fmt::Formatter, prefix: &str, is_left: bool) -> fmt::Result
            where
                C: PeekingCursor<'t> + 't,
                C::Item: $fmt_trait,
            {
                write!(f, "{prefix}")?;
                if is_left {
                    write!(f, "├──")?;
                } else {
                    write!(f, "└──")?;
                };
                if let Some(node) = cursor.get() {
                    node.fmt(f)?;
                    writeln!(f, "")?;
                    let new_prefix = String::from(prefix) + if is_left { "│  " } else { "   " };
                    let mut left_cursor = cursor.spawn_cursor();
                    let mut right_cursor = cursor.spawn_cursor();
                    if left_cursor.try_move_left() {
                        recursive_fmt(left_cursor, f, &new_prefix, true)?;
                    }
                    if right_cursor.try_move_right() {
                        recursive_fmt(right_cursor, f, &new_prefix, false)?;
                    }
                    Ok(())
                } else {
                    write!(f, "L\n")
                }
            }
            
            write!(f, "\n")?;
            recursive_fmt(tree.cursor(), f, "", false)
        }
    };
}

fmt_binary_tree!(fmt_debug_binary_tree, Debug);
fmt_binary_tree!(fmt_display_binary_tree, Display);
