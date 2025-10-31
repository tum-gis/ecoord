use crate::Error::{InvalidFileExtension, NoFileName};
use crate::ecoord::FILE_EXTENSION_ECOORD_FORMAT;
use crate::ecoord::read_impl::read_from_json_file;
use crate::error::Error;
use ecoord_core::ReferenceFrames;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// `EcoordReader` sets up a reader for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordReader<R: Read> {
    reader: R,
}

impl<R: Read> EcoordReader<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }

    pub fn finish(self) -> Result<ReferenceFrames, Error> {
        read_from_json_file(self.reader)
    }
}

impl EcoordReader<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let file_name_str = path
            .as_ref()
            .file_name()
            .ok_or(NoFileName())?
            .to_string_lossy()
            .to_lowercase();
        if !file_name_str.ends_with(FILE_EXTENSION_ECOORD_FORMAT) {
            return Err(InvalidFileExtension(file_name_str.to_string()));
        }

        let file = File::open(path)?;
        Ok(Self::new(file))
    }
}
