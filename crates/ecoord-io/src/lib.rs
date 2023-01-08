mod documents;
pub mod error;
mod read;
mod read_impl;
mod write;
mod write_impl;

#[doc(inline)]
pub use error::Error;

#[doc(inline)]
pub use read::EcoordReader;

#[doc(inline)]
pub use write::EcoordWriter;
