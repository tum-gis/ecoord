use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("data store disconnected")]
    InvalidChannelId(String), // TODO
}
