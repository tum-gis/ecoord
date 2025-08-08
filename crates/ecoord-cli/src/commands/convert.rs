use crate::error::Error;
use chrono::{DateTime, Utc};
use ecoord::io::{EcoordWriter, KittiReader};
use ecoord::{ChannelId, FrameId};
use nalgebra::Vector3;
use std::path::Path;
use tracing::info;

pub fn run(
    kitti_file_path: impl AsRef<Path>,
    ecoord_file_path: impl AsRef<Path>,
    start_date_time: DateTime<Utc>,
    stop_date_time: DateTime<Utc>,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    world_offset_channel_id: ChannelId,
    world_frame_id: FrameId,
    world_offset: Option<Vector3<f64>>,
) -> Result<(), Error> {
    info!(
        "Convert from KITTI at {}",
        kitti_file_path.as_ref().display()
    );

    let reference_frames = KittiReader::from_path(&kitti_file_path)?
        .with_trajectory_channel_id(trajectory_channel_id)
        .with_trajectory_frame_id(trajectory_frame_id)
        .with_trajectory_child_frame_id(trajectory_child_frame_id)
        .with_world_offset_channel_id(world_offset_channel_id)
        .with_world_frame_id(world_frame_id)
        .with_world_offset(world_offset)
        .finish(start_date_time, stop_date_time)?;

    EcoordWriter::from_path(&ecoord_file_path)?
        .with_pretty_write(true)
        .finish(&reference_frames)?;
    info!(
        "Completed conversion and writing to {}",
        ecoord_file_path.as_ref().display()
    );

    Ok(())
}
