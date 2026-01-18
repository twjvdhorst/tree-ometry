mod traversal_stack;
mod traversal_stack_mut;
pub mod inorder;
pub mod postorder;
pub mod preorder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkInstruction {
    Left,
    Right,
    Both,
    None,
}
