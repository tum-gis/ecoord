use crate::error::Error;
use chrono::{DateTime, Utc};
use ecoord_core::TransformTree;

pub fn interpolate_to_time(
    transform_tree: TransformTree,
    _timestamp: DateTime<Utc>,
) -> Result<(), Error> {
    Ok(())
}
