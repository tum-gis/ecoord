use std::io::{Read, Write};
use strum_macros::EnumIter;

pub const DEFAULT_COMPRESSION_LEVEL: i32 = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, EnumIter)]
pub enum Compression {
    #[default]
    None,
    Zstd(i32),
}

impl Compression {
    pub fn as_str(&self) -> Option<&'static str> {
        match self {
            Compression::None => None,
            Compression::Zstd(_) => Some("zst"),
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "zst" => Some(Compression::Zstd(DEFAULT_COMPRESSION_LEVEL)),
            _ => None,
        }
    }
}

impl Compression {
    pub const fn default_zstd() -> Self {
        Self::Zstd(DEFAULT_COMPRESSION_LEVEL)
    }

    pub fn is_enabled(&self) -> bool {
        !matches!(self, Compression::None)
    }

    pub fn level(&self) -> Option<i32> {
        match self {
            Compression::None => None,
            Compression::Zstd(level) => Some(*level),
        }
    }
}

impl Compression {
    pub fn wrap_reader<'a, R: Read + 'a>(
        &self,
        reader: R,
    ) -> Result<Box<dyn Read + 'a>, std::io::Error> {
        match self {
            Compression::None => Ok(Box::new(reader)),
            Compression::Zstd(_) => {
                let decoder = zstd::Decoder::new(reader)?;
                Ok(Box::new(decoder))
            }
        }
    }

    pub fn wrap_writer<'a, W: Write + 'a>(
        &self,
        writer: W,
    ) -> Result<Box<dyn Write + 'a>, std::io::Error> {
        match self {
            Compression::None => Ok(Box::new(writer)),
            Compression::Zstd(level) => {
                let encoder = zstd::Encoder::new(writer, *level)?;
                Ok(Box::new(encoder.auto_finish()))
            }
        }
    }
}
