use crate::Error;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::kitti::FILE_EXTENSION_KITTI_FORMAT;
use crate::kitti::read_impl::read_from_csv_file;
use chrono::{DateTime, Utc};
use ecoord_core::{FrameId, TransformTree};
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// `KittiReader` sets up a reader for reading KITTI pose files.
///
#[derive(Debug, Clone)]
pub struct KittiReader<R: Read> {
    reader: R,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    global_frame_id: FrameId,
    local_origin_offset: Option<nalgebra::Vector3<f64>>,
}

impl<R: Read> KittiReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            trajectory_frame_id: FrameId::local(),
            trajectory_child_frame_id: FrameId::base_link(),
            global_frame_id: FrameId::global(),
            local_origin_offset: None,
        }
    }

    pub fn with_trajectory_parent_frame_id(mut self, value: FrameId) -> Self {
        self.trajectory_frame_id = value;
        self
    }

    pub fn with_trajectory_child_frame_id(mut self, value: FrameId) -> Self {
        self.trajectory_child_frame_id = value;
        self
    }

    pub fn with_global_frame_id(mut self, value: FrameId) -> Self {
        self.global_frame_id = value;
        self
    }

    pub fn with_local_origin_offset(mut self, value: Option<nalgebra::Vector3<f64>>) -> Self {
        self.local_origin_offset = value;
        self
    }

    pub fn finish(
        self,
        start_date_time: DateTime<Utc>,
        end_date_time: DateTime<Utc>,
    ) -> Result<TransformTree, Error> {
        let transform_tree = read_from_csv_file(
            self.reader,
            start_date_time,
            end_date_time,
            self.trajectory_frame_id,
            self.trajectory_child_frame_id,
            self.global_frame_id,
            self.local_origin_offset,
        )?;

        Ok(transform_tree)
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
