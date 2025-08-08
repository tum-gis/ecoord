use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    EcoordError(#[from] ecoord_core::Error),
    #[error(transparent)]
    CsvError(#[from] csv::Error),

    #[error("file extension is invalid")]
    IsometryNotDerivable(),
}
