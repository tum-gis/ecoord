use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord_core::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Parsing(#[from] serde_json::Error),

    #[error(transparent)]
    KittiReader(#[from] crate::kitti::error::Error),

    #[error("file extension is invalid")]
    NoFileExtension(),
    #[error("file extension `{0}` is invalid")]
    InvalidFileExtension(String),
    #[error("file extension is invalid")]
    NoFileName(),
}
