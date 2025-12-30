use thiserror::Error;

#[derive(Debug, Error)]
pub enum StructureError {
    #[error("tree is a leaf")]
    EmptyTree,
}
