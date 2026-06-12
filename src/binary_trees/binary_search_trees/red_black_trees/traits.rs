use std::{borrow::Borrow, cmp::Ordering};

use super::Color;
use crate::binary_trees::{
    Side, 
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursorMut,
    }, 
    cursor_errors::CursorError,
};

pub(super) trait RedBlackCursor: PeekingCursorMut {
    type Key: Ord;
    type Value;

    fn key(&self) -> Option<&Self::Key>;
    
    fn color(&self) -> Option<Color>;
    fn parent_color(&self) -> Option<Color>;
    fn uncle_color(&self) -> Option<Color>;
    fn left_color(&self) -> Option<Color>;
    fn right_color(&self) -> Option<Color>;
    
    fn set_color(&mut self, color: Color);
    fn set_child_color(&mut self, side: Side, color: Color);

    fn move_up_after_subtree_change(&mut self) -> Option<Side> {
        self.move_up()
    }
    fn rotate(&mut self, side: Side) -> Result<(), CursorError>;

    fn attach_child(&mut self, key: Self::Key, value: Self::Value, color: Color, side: Side) -> Result<(), CursorError>;
    fn detach_node(&mut self) -> Option<(Self::Key, Self::Value)>;
    fn transplant_child(&mut self) -> Option<(Self::Key, Self::Value)>;

    fn swap_data_with_successor(&mut self);

    /// Moves the cursor to the direct predecessor or successor of the value being inserted.
    /// Reports the side of the node that the key should be inserted at, or None if the node contains the key already.
    fn find_node_to_insert_at(&mut self, key: &Self::Key) -> Option<Side> {
        while let Some(curr_key) = self.key() {
            match Self::Key::cmp(&key, curr_key) {
                Ordering::Less => {
                    if !self.try_move_left() {
                        return Some(Side::Left);
                    }
                },
                Ordering::Greater => {
                    if !self.try_move_right() {
                        return Some(Side::Right);
                    }
                },
                Ordering::Equal => {
                    return None;
                },
            };
        }
        None
    }

    fn insert_fixup(&mut self) {
        // Cormen et al.'s algorithm.
        // We maintain the invariant that all nodes below the cursor have the correct semigroup value.
        while self.parent_color() == Some(Color::Red) {
            // Throughout the loop, cursor points to z, and peeking_cursor moves around to check states of various nodes.
            let side_current = self.move_up_after_subtree_change().unwrap(); // Move the cursor to z.p
            let side_parent = self.side_of_parent().unwrap(); // Can unwrap safely, as z.p.p exists by the proof of correctness by Cormen et al.

            if self.uncle_color() == Some(Color::Red) {
                // Case 1
                self.set_color(Color::Black);
                self.move_up_after_subtree_change(); // Move the cursor to z.p.p, where it stays for the next iteration.
                self.set_color(Color::Red);
                self.set_child_color(side_parent.opposite(), Color::Black);
            } else {
                if side_current == side_parent.opposite() {
                    // Case 2
                    self.rotate(side_parent).unwrap();
                    self.move_up_after_subtree_change();
                }

                // Case 3
                self.set_color(Color::Black);
                self.move_up_after_subtree_change();
                self.set_color(Color::Red);
                self.rotate(side_parent.opposite()).unwrap();

                // After rotating around z.p.p, z is the sibling of the node pointed at by the cursor.
                let side = self.move_up_after_subtree_change().unwrap();
                self.move_side(side.opposite());
            }
        }
    }

    fn remove_fixup(&mut self) {
        todo!()
    }
}

pub(super) trait RedBlackTree {
    type Key: Ord;
    type Value;
    type RbCursor: RedBlackCursor<Key = Self::Key, Value = Self::Value>;

    fn rb_cursor(&mut self) -> Self::RbCursor;

    fn insert(&mut self, key: Self::Key, value: Self::Value) -> Option<Self::Value> {
        todo!()
    }

    fn get_cursor_mut_at_key<Q>(&mut self, key: &Q) -> Option<Self::RbCursor>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord + ?Sized,
    {
        todo!()
    }

    fn remove_entry<Q>(&mut self, key: &Q) -> Option<(Self::Key, Self::Value)>
    where 
        Self::Key: Borrow<Q>,
        Q: Ord,
    {
        todo!()
    }
}
