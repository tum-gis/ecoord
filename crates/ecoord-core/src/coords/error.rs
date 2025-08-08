use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
    //#[error("data store disconnected")]
    //InvalidChannelId(ChannelId),
    #[error("No row indices specified")]
    LowerBoundExceedsUpperBound,

    #[error(
        "octree level `{level}` only allows indices of up to `{maximum_index}`, but found ({x}, {y}, {z})"
    )]
    IndexOutOfBounds {
        level: u32,
        maximum_index: u64,
        x: u64,
        y: u64,
        z: u64,
    },
    #[error("index too large for this representation")]
    IndexTooLarge,
    #[error("path is not a directory")]
    InvalidNumber,

    #[error("path is not a directory")]
    CellAlreadyOccupied,

    #[error("path is not a directory")]
    NoMinValue,
    #[error("path is not a directory")]
    NoMaxValue,
}
