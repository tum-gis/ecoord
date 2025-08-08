use std::fmt;

/// Dedicated type for an identifier of a frame.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FrameId(String);

impl fmt::Display for FrameId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<FrameId> for String {
    fn from(item: FrameId) -> Self {
        item.0
    }
}

impl From<String> for FrameId {
    fn from(item: String) -> Self {
        Self(item)
    }
}

impl From<&str> for FrameId {
    fn from(item: &str) -> Self {
        Self(item.to_string())
    }
}

/// Additional information for a frame.
///
///
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct FrameInfo {
    pub crs_epsg: Option<u32>,
}

impl FrameInfo {
    pub fn new(crs_epsg: Option<u32>) -> Self {
        Self { crs_epsg }
    }
}
