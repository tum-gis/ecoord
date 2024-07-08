use crate::error::Error;
use crate::read_impl::read_from_json_file;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::FILE_EXTENSION_ECOORD_FORMAT;
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
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_ECOORD_FORMAT {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = File::open(path)?;
        Ok(Self::new(file))
    }
}
