use thiserror::Error;

use super::Side;

#[derive(Error, Debug)]
pub enum CursorError {
    #[error("cursor does not point to a node")]
    InvalidCursor,
    #[error("node already has a child node on side {0}")]
    CannotAttachChild(Side),
    #[error("cannot delete node with children")]
    CannotDetachNode,
}
