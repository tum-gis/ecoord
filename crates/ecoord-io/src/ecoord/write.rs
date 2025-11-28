use crate::Compression;
use crate::Error::{InvalidFileExtension, NoFileName};
use crate::ecoord::format::Format;
use crate::ecoord::write_impl::write_to_json_file;
use crate::error::Error;
use ecoord_core::TransformTree;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

/// `EcoordWriter` sets up a writer for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordWriter<W: Write> {
    writer: W,
    compression: Compression,
    format: Format,
    pretty: bool,
}

impl<W: Write> EcoordWriter<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            compression: Compression::None,
            format: Format::Json,
            pretty: false,
        }
    }

    pub fn with_compression(mut self, compression: Compression) -> Self {
        self.compression = compression;
        self
    }

    pub fn with_format(mut self, format: Format) -> Self {
        self.format = format;
        self
    }

    pub fn with_pretty(mut self, pretty: bool) -> Self {
        self.pretty = pretty;
        self
    }

    pub fn finish(self, transform_tree: &TransformTree) -> Result<(), Error> {
        let buffered_writer = BufWriter::new(self.writer);
        let writer = self.compression.wrap_writer(buffered_writer)?;

        match self.format {
            Format::Json => {
                write_to_json_file(writer, self.pretty, transform_tree)?;
            }
            Format::Csv => {
                unimplemented!("writing a CSV not supported yet")
            }
        }

        Ok(())
    }
}

impl EcoordWriter<File> {
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, Error> {
        // Detect compression from the final extension
        let extension = path
            .as_ref()
            .extension()
            .ok_or(NoFileName())?
            .to_str()
            .ok_or(NoFileName())?;
        let compression = Compression::from_str(extension).unwrap_or_default();

        // Detect the format by stripping compression extension if present
        let format_part = if compression != Compression::None {
            path.as_ref()
                .file_stem()
                .and_then(|s| Path::new(s).extension())
                .and_then(|s| s.to_str())
        } else {
            Some(extension)
        };

        let format = format_part.and_then(Format::from_str).ok_or_else(|| {
            let file_name = path
                .as_ref()
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());
            InvalidFileExtension(file_name)
        })?;

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)?;
        Ok(Self::new(file)
            .with_compression(compression)
            .with_format(format))
    }
}
