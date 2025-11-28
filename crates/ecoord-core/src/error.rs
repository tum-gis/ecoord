use crate::{FrameId, TransformId};
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    CoordsError(#[from] crate::coords::error::Error),

    #[error("transform with id `{0}` not available")]
    InvalidTransformId(TransformId),

    #[error("invalid frame id `{0}`")]
    InvalidFrameId(FrameId),

    #[error("no transforms")]
    NoTransforms(),

    #[error("no transforms")]
    ContainsDynamicTransform(),

    #[error("duplicate timestamp `{0}`")]
    DuplicateTimestamp(DateTime<Utc>),

    #[error("no transform path found for `{0}`")]
    NoTransformPath(TransformId),

    #[error("multiple transform path found for `{0}`")]
    MultipleTransformPaths(TransformId),

    #[error("no timestamp")]
    MissingTimestamp(),

    #[error("transforms not sorted by timestamp")]
    TransformsNotSortedByTime(),

    #[error("transforms must be sorted strictly ascending by timestamp")]
    TransformsNotSorted { transform_id: TransformId },

    #[error("collision")]
    ChannelTransformCollisions { transform_id: TransformId },
}
