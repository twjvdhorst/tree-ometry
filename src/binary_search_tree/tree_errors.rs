use thiserror::Error;

#[derive(Clone, Copy, Debug, Error, PartialEq, Eq)]
pub enum StructureError {
    #[error("tree is a leaf")]
    EmptyTree,
}
