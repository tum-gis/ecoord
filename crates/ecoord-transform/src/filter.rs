use crate::error::Error;
use chrono::{DateTime, Utc};
use ecoord_core::{ChannelId, ReferenceFrames, Transform, TransformId};
use std::collections::HashMap;

pub fn interpolate_to_time(
    reference_frames: ReferenceFrames,
    _timestamp: DateTime<Utc>,
) -> Result<(), Error> {
    let _filtered_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = reference_frames
        .transforms()
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        //.filter(|t| true)
        .collect();

    Ok(())
}
