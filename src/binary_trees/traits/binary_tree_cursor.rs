pub trait BinaryTreeCursor: Sized {
    type Node;

    fn node(&self) -> Option<&Self::Node>;

    fn move_up(&mut self) -> Option<&Self::Node>;
    fn move_left(&mut self) -> Option<&Self::Node>;
    fn move_right(&mut self) -> Option<&Self::Node>;
}

pub(crate) trait BinaryTreeCursorMut: BinaryTreeCursor {
    fn node_mut(&mut self) -> Option<&mut Self::Node>;

    fn move_up_mut(&mut self) -> Option<&mut Self::Node>;
    fn move_left_mut(&mut self) -> Option<&mut Self::Node>;
    fn move_right_mut(&mut self) -> Option<&mut Self::Node>;
}
