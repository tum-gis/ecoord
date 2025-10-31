use crate::Error;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::tabular::{FILE_EXTENSION_TABULAR_CSV_FORMAT, read_impl};
use ecoord_core::{ChannelId, FrameId, ReferenceFrames};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub const TRAJECTORY_CHANNEL_ID: &str = "trajectory";
pub const TRAJECTORY_FRAME_ID: &str = "world";
pub const TRAJECTORY_CHILD_FRAME_ID: &str = "body";

/// `TabularReader` sets up a reader for tabular files.
///
#[derive(Debug, Clone)]
pub struct TabularReader<R: Read> {
    reader: R,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
}

impl<R: Read> TabularReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            trajectory_channel_id: ChannelId::from(TRAJECTORY_CHANNEL_ID),
            trajectory_frame_id: FrameId::from(TRAJECTORY_FRAME_ID),
            trajectory_child_frame_id: FrameId::from(TRAJECTORY_CHILD_FRAME_ID),
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

    pub fn finish(self) -> Result<ReferenceFrames, Error> {
        let reference_frames = read_impl::read_from_csv_file(
            self.reader,
            self.trajectory_channel_id,
            self.trajectory_frame_id,
            self.trajectory_child_frame_id,
        )?;

        Ok(reference_frames)
    }
}

impl TabularReader<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_TABULAR_CSV_FORMAT {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = File::open(path)?;
        Ok(Self::new(file))
    }
}
