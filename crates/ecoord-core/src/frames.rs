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

/// Identifiers using `FrameId::from()`.
impl FrameId {
    /// Standard global reference frame identifier.
    #[inline]
    pub fn global() -> FrameId {
        FrameId::from("global")
    }

    /// Standard local reference frame identifier.
    #[inline]
    pub fn local() -> FrameId {
        FrameId::from("local")
    }

    /// Standard base_link reference frame identifier.
    #[inline]
    pub fn base_link() -> FrameId {
        FrameId::from("base_link")
    }

    /// Map frame - typically a static map coordinate frame.
    #[inline]
    pub fn map() -> FrameId {
        FrameId::from("map")
    }

    /// Submap frame - frame for a portion of a larger map.
    #[inline]
    pub fn submap() -> FrameId {
        FrameId::from("submap")
    }

    /// Platform frame - frame of the platform, e.g. vehicle, etc.
    #[inline]
    pub fn platform() -> FrameId {
        FrameId::from("platform")
    }

    /// Odometry frame - frame for odometry data.
    #[inline]
    pub fn odom() -> FrameId {
        FrameId::from("odom")
    }

    /// Sensor frame - frame of a sensor, e.g. camera, lidar, etc.
    #[inline]
    pub fn sensor() -> FrameId {
        FrameId::from("sensor")
    }
}

/// Additional information for a frame.
///
///
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FrameInfo {
    pub id: FrameId,
    pub description: Option<String>,
    pub crs_epsg: Option<u32>,
}

impl FrameInfo {
    pub fn new(id: FrameId, description: Option<String>, crs_epsg: Option<u32>) -> Self {
        Self {
            id,
            description,
            crs_epsg,
        }
    }
}
