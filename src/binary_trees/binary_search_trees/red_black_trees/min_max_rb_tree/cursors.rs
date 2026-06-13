use crate::binary_trees::{
    Neighborhood, Side, binary_search_trees::{
        min_max_rb_tree::MinMaxRbData,
        red_black_trees::base,
    },
    binary_tree::NodeId,
    binary_tree_cursor::{
        BinaryTreeCursor,
        PeekingCursor,
        PeekingCursorMut,
    },
};

/// A cursor over a SemigroupRbTree.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub struct Cursor<'t, K, V>(base::Cursor<'t, MinMaxRbData<K, V>>);

/// Make own implementation of Clone, so K and V don't have to be Clone.
impl<'t, K, V> Clone for Cursor<'t, K, V> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<'t, K, V> Copy for Cursor<'t, K, V> {}

impl<'t, K, V> From<CursorMut<'t, K, V>> for Cursor<'t, K, V> {
    fn from(value: CursorMut<'t, K, V>) -> Self {
        Self(value.0.into())
    }
}

impl<'t, K, V> Cursor<'t, K, V> {
    pub(super) fn new(cursor: base::Cursor<'t, MinMaxRbData<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn into_inner(self) -> base::Cursor<'t, MinMaxRbData<K, V>> {
        self.0
    }

    pub(crate) fn move_to_id(&mut self, node_id: NodeId) {
        self.0.move_to_id(node_id)
    }

    pub fn subtree_min(&self) -> Option<&K> {
        let mut peeking_cursor = self.clone();
        peeking_cursor.move_to_id(self.0.get()?.id_min());
        peeking_cursor.get().map(|(k, _)| k)
    }

    pub fn subtree_max(&self) -> Option<&K> {
        let mut peeking_cursor = self.clone();
        peeking_cursor.move_to_id(self.0.get()?.id_max());
        peeking_cursor.get().map(|(k, _)| k)
    }
}

impl<'t, K, V> BinaryTreeCursor for Cursor<'t, K, V> {   
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

impl<'t, K, V> PeekingCursor for Cursor<'t, K, V> {
    type Item = (&'t K, &'t V);

    fn get(&self) -> Option<Self::Item> {
        self.0.get().map(MinMaxRbData::data)
    }

    fn peek_up(&self) -> Option<Self::Item> {
        self.0.peek_up().map(MinMaxRbData::data)
    }

    fn peek_left(&self) -> Option<Self::Item> {
        self.0.peek_left().map(MinMaxRbData::data)
    }

    fn peek_right(&self) -> Option<Self::Item> {
        self.0.peek_right().map(MinMaxRbData::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item> {
        self.0.peek_neighborhood().map(MinMaxRbData::data)
    }
}

/// A cursor over a SemigroupRbTree with editing operations.
/// A Cursor can freely walk through the tree.
/// When created, Cursors start at the (possibly non-existent) root of the tree.
pub struct CursorMut<'t, K, V>(base::CursorMut<'t, MinMaxRbData<K, V>>);

impl<'t, K, V> CursorMut<'t, K, V> {
    pub(super) fn new(cursor: base::CursorMut<'t, MinMaxRbData<K, V>>) -> Self {
        Self(cursor)
    }

    pub(super) fn into_inner(self) -> base::CursorMut<'t, MinMaxRbData<K, V>> {
        self.0
    }

    pub fn subtree_min(&self) -> Option<&K> {
        let mut peeking_cursor = self.as_cursor();
        peeking_cursor.move_to_id(self.0.get()?.id_min());
        peeking_cursor.get().map(|(k, _)| k)
    }

    pub fn subtree_max(&self) -> Option<&K> {
        let mut peeking_cursor = self.as_cursor();
        peeking_cursor.move_to_id(self.0.get()?.id_max());
        peeking_cursor.get().map(|(k, _)| k)
    }
}

impl<'t, K, V> BinaryTreeCursor for CursorMut<'t, K, V> {   
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

impl<'t, K, V> PeekingCursorMut for CursorMut<'t, K, V> {
    type Item<'c> = (&'c K, &'c V) where Self: 'c;
    type ItemMut<'c> = (&'c K, &'c mut V) where Self: 'c;
    type AsCursor<'c> = Cursor<'c, K, V> where Self: 'c;

    fn get(&self) -> Option<Self::Item<'_>> {
        self.0.get().map(MinMaxRbData::data)
    }

    fn get_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.get_mut().map(MinMaxRbData::data_with_mut_value)
    }

    fn as_cursor(&self) -> Self::AsCursor<'_> {
        Cursor::new(self.0.as_cursor())
    }

    fn peek_up(&self) -> Option<Self::Item<'_>> {
        self.0.peek_up().map(MinMaxRbData::data)
    }

    fn peek_left(&self) -> Option<Self::Item<'_>> {
        self.0.peek_left().map(MinMaxRbData::data)
    }

    fn peek_right(&self) -> Option<Self::Item<'_>> {
        self.0.peek_right().map(MinMaxRbData::data)
    }

    fn peek_neighborhood(&self) -> Neighborhood<Self::Item<'_>> {
        self.0.peek_neighborhood().map(MinMaxRbData::data)
    }

    fn peek_up_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_up_mut().map(MinMaxRbData::data_with_mut_value)
    }

    fn peek_left_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_left_mut().map(MinMaxRbData::data_with_mut_value)
    }

    fn peek_right_mut(&mut self) -> Option<Self::ItemMut<'_>> {
        self.0.peek_right_mut().map(MinMaxRbData::data_with_mut_value)
    }

    fn peek_neighborhood_mut(&mut self) -> Neighborhood<Self::ItemMut<'_>> {
        self.0.peek_neighborhood_mut().map(MinMaxRbData::data_with_mut_value)
    }
}
