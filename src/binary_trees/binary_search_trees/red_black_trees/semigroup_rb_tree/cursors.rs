use derive_more::Debug;

use crate::binary_trees::{
    Neighborhood,
    Side,
    binary_search_trees::{
        red_black_trees::base,
        semigroup_rb_tree::semigroup_rb_tree::SemigroupRbData,
    },
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
#[derive(Debug)]
pub struct Cursor<'t, K, V, S>(base::Cursor<'t, SemigroupRbData<K, V, S>>);

/// Make own implementation of Clone, so K, V, and S don't have to be Clone.
impl<'t, K, V, S> Clone for Cursor<'t, K, V, S> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V, S> Copy for Cursor<'t, K, V, S> {}

impl<'t, K, V, S> From<CursorMut<'t, K, V, S>> for Cursor<'t, K, V, S> {
    fn from(value: CursorMut<'t, K, V, S>) -> Self {
        Self(value.0.into())
    }
}

impl<'t, K, V, S> Cursor<'t, K, V, S> {
    pub(super) fn new(cursor: base::Cursor<'t, SemigroupRbData<K, V, S>>) -> Self {
        Self(cursor)
    }
}

impl<'t, K, V, S> BinaryTreeCursor for Cursor<'t, K, V, S> {   
    fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    fn move_left(&mut self) {
        self.0.move_left();
    }

    fn move_right(&mut self) {
        self.0.move_right();
    }
}

impl<'t, K, V, S> PeekingCursor for Cursor<'t, K, V, S> {
    type Item = (&'t K, &'t V, &'t S);

    fn get(&self) -> Option<Self::Item> {
        self.0.get().map(SemigroupRbData::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(SemigroupRbData::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(SemigroupRbData::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(SemigroupRbData::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood().map(SemigroupRbData::data)
    }
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
/// Cursors maintain the invariant that as long as the tree has a node, the cursor points to a node.
#[derive(Debug)]
pub struct CursorMut<'t, K, V, S>(base::CursorMut<'t, SemigroupRbData<K, V, S>>);

impl<'t, K, V, S> CursorMut<'t, K, V, S> {
    pub(super) fn new(cursor: base::CursorMut<'t, SemigroupRbData<K, V, S>>) -> Self {
        Self(cursor)
    }
}

impl<'t, K, V, S> BinaryTreeCursor for CursorMut<'t, K, V, S> {   
    fn side_of_parent(&self) -> Option<Side> {
        self.0.side_of_parent()
    }

    fn try_move_up(&mut self) -> Option<Side> {
        self.0.try_move_up()
    }
    
    fn try_move_left(&mut self) -> bool {
        self.0.try_move_left()
    }
    
    fn try_move_right(&mut self) -> bool {
        self.0.try_move_right()
    }

    fn move_up(&mut self) -> Option<Side> {
        self.0.move_up()
    }

    fn move_left(&mut self) {
        self.0.move_left();
    }

    fn move_right(&mut self) {
        self.0.move_right();
    }
}

impl<'t, K, V, S> PeekingCursorMut for CursorMut<'t, K, V, S> {
    type Item<'c> = (&'c K, &'c V, &'c S) where Self: 'c;
    type ItemMut<'c> = (&'c K, &'c mut V, &'c S) where Self: 'c;
    type AsCursor<'c> = Cursor<'c, K, V, S> where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>> {
        self.0.get().map(SemigroupRbData::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(SemigroupRbData::data_with_mut_value)
    }

    fn as_cursor(&self) -> Self::AsCursor<'_> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(SemigroupRbData::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(SemigroupRbData::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(SemigroupRbData::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(SemigroupRbData::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(SemigroupRbData::data_with_mut_value)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(SemigroupRbData::data_with_mut_value)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(SemigroupRbData::data_with_mut_value)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(SemigroupRbData::data_with_mut_value)
    }
}
