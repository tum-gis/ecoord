use crate::error::Error;
use crate::write_impl::write_to_json_file;
use ecoord_core::ReferenceFrames;
use std::path::{Path, PathBuf};

/// `EcoordWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordWriter {
    path: PathBuf,
}

impl EcoordWriter {
    pub fn new(path: impl AsRef<Path>) -> Self {
        //assert!(path.is_dir());
        Self {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn finish(&self, reference_frames: &ReferenceFrames) -> Result<(), Error> {
        write_to_json_file(&self.path, reference_frames)?;
        Ok(())
    }
}
