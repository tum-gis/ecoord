use crate::{ChannelId, FrameId, TransformId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CoordsError(#[from] crate::coords::error::Error),

    #[error("invalid channel id `{0}`")]
    InvalidChannelId(ChannelId),
    #[error("invalid channel ids")]
    InvalidChannelIds(Vec<ChannelId>),
    #[error("transform with id `{1}` not available for channel with id `{0}`")]
    InvalidTransformId(ChannelId, TransformId),

    #[error("invalid frame id `{0}`")]
    InvalidFrameId(FrameId),

    #[error("no channels")]
    NoChannels(),
    #[error("no transforms")]
    NoTransforms(),

    #[error("no transform path found for `{0}`")]
    NoTransformPath(TransformId),

    #[error("multiple transform path found for `{0}`")]
    MultipleTransformPaths(TransformId),

    #[error("no timestamp")]
    MissingTimestamp(),

    #[error("transforms not sorted by timestamp")]
    TransformsNotSortedByTime(),

    #[error("transforms must be sorted strictly ascending by timestamp")]
    TransformsNotSorted {
        channel_id: ChannelId,
        transform_id: TransformId,
    },

    #[error("collision")]
    ChannelTransformCollisions {
        channel_id: ChannelId,
        transform_id: TransformId,
    },
}
