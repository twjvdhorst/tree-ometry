use thiserror::Error;

use super::Side;

#[derive(Error, Debug)]
pub enum CursorError {
    #[error("cursor does not point at a node")]
    NullError,
    #[error("tree already has a root")]
    CreateRootError,
    #[error("node already has a parent node")]
    AttachParentError,
    #[error("node already has a child node on side {0}")]
    AttachChildError(Side),
    #[error("cannot delete node with children")]
    DetachError,
    #[error("cannot rotate left around cursor")]
    RotateLeftError,
    #[error("cannot rotate right around cursor")]
    RotateRightError,
}
