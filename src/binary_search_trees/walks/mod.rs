pub mod postorder;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkInstruction {
    Left,
    Right,
    Both,
    None,
}
