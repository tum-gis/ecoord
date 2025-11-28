use crate::error::Error;
use chrono::{DateTime, Utc};
use ecoord::FrameId;
use ecoord::io::{EcoordWriter, KittiReader};
use nalgebra::Vector3;
use std::path::Path;
use tracing::info;

pub fn run(
    kitti_file_path: impl AsRef<Path>,
    ecoord_file_path: impl AsRef<Path>,
    start_date_time: DateTime<Utc>,
    end_date_time: DateTime<Utc>,
    trajectory_parent_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    global_frame_id: FrameId,
    local_origin_offset: Option<Vector3<f64>>,
    pretty: bool,
) -> Result<(), Error> {
    info!(
        "Convert from KITTI at {}",
        kitti_file_path.as_ref().display()
    );

    let transform_tree = KittiReader::from_path(&kitti_file_path)?
        .with_trajectory_parent_frame_id(trajectory_parent_frame_id)
        .with_trajectory_child_frame_id(trajectory_child_frame_id)
        .with_global_frame_id(global_frame_id)
        .with_local_origin_offset(local_origin_offset)
        .finish(start_date_time, end_date_time)?;

    EcoordWriter::from_path(&ecoord_file_path)?
        .with_pretty(pretty)
        .finish(&transform_tree)?;
    info!(
        "Completed conversion and writing to {}",
        ecoord_file_path.as_ref().display()
    );

    Ok(())
}
