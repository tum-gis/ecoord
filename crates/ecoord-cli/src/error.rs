use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord::Error),
    #[error(transparent)]
    EcoordIoError(#[from] ecoord::io::Error),

    #[error(transparent)]
    StdIoError(#[from] std::io::Error),
}
