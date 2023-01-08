use std::fmt;

/// Dedicated type for an identifier of a channel.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelId(String);

impl fmt::Display for ChannelId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ChannelId> for String {
    fn from(item: ChannelId) -> Self {
        item.0
    }
}

impl From<String> for ChannelId {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<&str> for ChannelId {
    fn from(item: &str) -> Self {
        Self(item.to_string())
    }
}

/// Additional information for a frame.
///
///
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ChannelInfo {
    pub priority: Option<i32>,
}

impl ChannelInfo {
    pub fn new(priority: Option<i32>) -> Self {
        Self { priority }
    }
}
