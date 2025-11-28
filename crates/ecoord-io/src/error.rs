use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord_core::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),
    #[error(transparent)]
    ChronoParseError(#[from] chrono::ParseError),

    #[error(transparent)]
    KittiReaderError(#[from] crate::kitti::error::Error),

    #[error("file extension is invalid")]
    NoFileExtension(),
    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("file extension is invalid")]
    NoFileName(),
    #[error("multiple files found")]
    MultipleFiles(),

    #[error("timestamp is missing")]
    NoTimestamp(),
    #[error("timestamp is missing")]
    TimestampDefinedTwice(),
    #[error("timestamp is missing")]
    InvalidTimestamp(),
}
