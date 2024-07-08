mod documents;
mod error;
mod read;
mod read_impl;
mod write;
mod write_impl;

#[doc(inline)]
pub use crate::error::Error;

#[doc(inline)]
pub use crate::read::EcoordReader;

#[doc(inline)]
pub use crate::write::EcoordWriter;

pub const FILE_EXTENSION_ECOORD_FORMAT: &str = "json";
