use crate::error::Error;
use crate::write_impl::write_to_json_file;
use crate::Error::{InvalidFileExtension, NoFileExtension};
use crate::FILE_EXTENSION_ECOORD_FORMAT;
use ecoord_core::ReferenceFrames;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;

/// `EcoordWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordWriter<W: Write> {
    writer: W,
    pretty_write: bool,
}

impl<W: Write> EcoordWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            pretty_write: false,
        }
    }

    pub fn with_pretty_write(mut self, pretty_write: bool) -> Self {
        self.pretty_write = pretty_write;
        self
    }

    pub fn finish(self, reference_frames: &ReferenceFrames) -> Result<(), Error> {
        write_to_json_file(self.writer, self.pretty_write, reference_frames)?;
        Ok(())
    }
}

impl EcoordWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        let extension = path.as_ref().extension().ok_or(NoFileExtension())?;
        if extension != FILE_EXTENSION_ECOORD_FORMAT {
            return Err(InvalidFileExtension(
                extension.to_str().unwrap_or_default().to_string(),
            ));
        }

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file))
    }
}
