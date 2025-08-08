use crate::Error;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::kitti::FILE_EXTENSION_KITTI_FORMAT;
use crate::kitti::read_impl::read_from_csv_file;
use chrono::{DateTime, Utc};
use ecoord_core::{ChannelId, FrameId, ReferenceFrames};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const TRAJECTORY_CHANNEL_ID: &str = "slam";
pub const TRAJECTORY_FRAME_ID: &str = "world_offset";
pub const TRAJECTORY_CHILD_FRAME_ID: &str = "base_link";
pub const WORLD_OFFSET_CHANNEL_ID: &str = "world_offset";
pub const WORLD_FRAME_ID: &str = "world";

/// `KittiReader` sets up a reader for reading KITTI pose files.
///
#[derive(Debug, Clone)]
pub struct KittiReader<R: Read> {
    reader: R,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    world_offset_channel_id: ChannelId,
    world_frame_id: FrameId,
    world_offset: Option<nalgebra::Vector3<f64>>,
}

impl<R: Read> KittiReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            trajectory_channel_id: ChannelId::from(TRAJECTORY_CHANNEL_ID),
            trajectory_frame_id: FrameId::from(TRAJECTORY_FRAME_ID),
            trajectory_child_frame_id: FrameId::from(TRAJECTORY_CHILD_FRAME_ID),
            world_offset_channel_id: ChannelId::from(WORLD_OFFSET_CHANNEL_ID),
            world_frame_id: FrameId::from(WORLD_FRAME_ID),
            world_offset: None,
        }
    }

    pub fn with_trajectory_channel_id(mut self, value: ChannelId) -> Self {
        self.trajectory_channel_id = value;
        self
    }

    pub fn with_trajectory_frame_id(mut self, value: FrameId) -> Self {
        self.trajectory_frame_id = value;
        self
    }

    pub fn with_trajectory_child_frame_id(mut self, value: FrameId) -> Self {
        self.trajectory_child_frame_id = value;
        self
    }

    pub fn with_world_offset_channel_id(mut self, value: ChannelId) -> Self {
        self.world_offset_channel_id = value;
        self
    }

    pub fn with_world_frame_id(mut self, value: FrameId) -> Self {
        self.world_frame_id = value;
        self
    }

    pub fn with_world_offset(mut self, value: Option<nalgebra::Vector3<f64>>) -> Self {
        self.world_offset = value;
        self
    }

    pub fn finish(
        self,
        start_date_time: DateTime<Utc>,
        stop_date_time: DateTime<Utc>,
    ) -> Result<ReferenceFrames, Error> {
        let reference_frames = read_from_csv_file(
            self.reader,
            start_date_time,
            stop_date_time,
            self.trajectory_channel_id,
            self.trajectory_frame_id,
            self.trajectory_child_frame_id,
            self.world_offset_channel_id,
            self.world_frame_id,
            self.world_offset,
        )?;

        Ok(reference_frames)
    }
}

impl KittiReader<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_KITTI_FORMAT {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = File::open(path)?;
        Ok(Self::new(file))
    }
}
