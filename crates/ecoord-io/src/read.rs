use crate::error::Error;
use crate::read_impl::read_from_json_file;
use ecoord_core::ReferenceFrames;
use std::path::{Path, PathBuf};

/// `EcoordReader` sets up a reader for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordReader {
    path: PathBuf,
}

impl EcoordReader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn finish(self) -> Result<ReferenceFrames, Error> {
        read_from_json_file(&self.path)
    }
}
