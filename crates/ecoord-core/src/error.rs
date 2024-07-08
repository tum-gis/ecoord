use crate::{ChannelId, FrameId, TransformId};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    InvalidChannelId(ChannelId),
    #[error("data store disconnected")]
    InvalidChannelIds(Vec<ChannelId>),
    #[error("transform with id `{1}` not available for channel with id `{0}`")]
    InvalidTransformId(ChannelId, TransformId),

    #[error("frame id unknown")]
    InvalidFrameId(FrameId),

    #[error("data store disconnected")]
    NoChannels(),
    #[error("data store disconnected")]
    MissingTransforms(),

    #[error("transforms must be sorted strictly ascending by timestamp")]
    MissingTimestamp(),

    #[error("transforms must be sorted strictly ascending by timestamp")]
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
