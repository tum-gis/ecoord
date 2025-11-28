use crate::Compression;
use crate::Error::{InvalidFileExtension, NoFileName};
use crate::ecoord::FILE_EXTENSION_ECOORD_FORMAT;
use crate::ecoord::format::Format;
use crate::ecoord::read_impl::{read_from_csv_file, read_from_json_file};
use crate::error::Error;
use ecoord_core::TransformTree;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};
use strum::IntoEnumIterator;

/// `EcoordReader` sets up a reader for the custom reader data structure.
///
#[derive(Debug, Clone)]
pub struct EcoordReader<R: Read> {
    reader: R,
    compression: Compression,
    format: Format,
}

impl<R: Read> EcoordReader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            compression: Compression::None,
            format: Format::Json,
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

    pub fn finish(self) -> Result<TransformTree, Error> {
        let buffered_reader = BufReader::new(self.reader);
        let reader = self.compression.wrap_reader(buffered_reader)?;

        match self.format {
            Format::Json => read_from_json_file(reader),
            Format::Csv => read_from_csv_file(reader),
        }
    }
}

impl EcoordReader<File> {
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

        let file = File::open(path)?;
        Ok(Self::new(file)
            .with_compression(compression)
            .with_format(format))
    }

    pub fn from_base_path(
        base_path: impl AsRef<Path>,
        file_name: impl AsRef<str>,
    ) -> Result<Option<Self>, Error> {
        let base_path = base_path.as_ref();
        let file_name = file_name.as_ref();

        // Generate all possible path combinations
        let mut existing_paths: Vec<PathBuf> = Vec::new();
        for format in Format::iter() {
            for compression in Compression::iter() {
                let extension = format.as_str();

                let full_name = if let Some(c) = compression.as_str() {
                    format!(
                        "{}.{FILE_EXTENSION_ECOORD_FORMAT}.{}.{}",
                        file_name, extension, c
                    )
                } else {
                    format!("{}.{FILE_EXTENSION_ECOORD_FORMAT}.{}", file_name, extension)
                };

                let path = base_path.join(&full_name);
                if path.exists() {
                    existing_paths.push(path);
                }
            }
        }

        match existing_paths.len() {
            0 => Ok(None),
            1 => Ok(Some(Self::from_path(&existing_paths[0])?)),
            _ => Err(Error::MultipleFiles()),
        }
    }
}
