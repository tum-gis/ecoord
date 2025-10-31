use crate::Error::{InvalidFileExtension, NoFileName};
use crate::ecoord::FILE_EXTENSION_ECOORD_FORMAT;
use crate::ecoord::write_impl::write_to_json_file;
use crate::error::Error;
use ecoord_core::ReferenceFrames;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// `EcoordWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordWriter<W: Write> {
    writer: W,
    pretty: bool,
}

impl<W: Write> EcoordWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            pretty: false,
        }
    }

    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    pub fn finish(self, reference_frames: &ReferenceFrames) -> Result<(), Error> {
        write_to_json_file(self.writer, self.pretty, reference_frames)?;
        Ok(())
    }
}

impl EcoordWriter<File> {
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

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file))
    }
}
