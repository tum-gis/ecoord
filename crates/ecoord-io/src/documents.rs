use chrono::{DateTime, Duration, TimeZone, Timelike, Utc};
use nalgebra::{Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct EcoordDocument {
    pub transforms: Vec<TransformElement>,
    pub frame_info: Vec<FrameInfoElement>,
    pub channel_info: Vec<ChannelInfoElement>,
    pub transform_info: Vec<TransformInfoElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FrameInfoElement {
    pub id: String,
    pub crs_epsg: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct ChannelInfoElement {
    pub id: String,
    pub priority: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct TransformInfoElement {
    pub frame_id: String,
    pub child_frame_id: String,
    pub interpolation_method: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TransformElement {
    pub channel_id: String,
    pub frame_id: String,
    pub child_frame_id: String,
    pub timestamp: TimeElement,
    pub duration: Option<DurationElement>,
    pub translation: VectorElement,
    pub rotation: QuaternionElement,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct TimeElement {
    sec: i64,
    nanosec: u32,
}

impl From<TimeElement> for DateTime<Utc> {
    fn from(item: TimeElement) -> Self {
        Utc.timestamp_opt(item.sec, item.nanosec).unwrap()
    }
}

impl From<DateTime<Utc>> for TimeElement {
    fn from(item: DateTime<Utc>) -> Self {
        Self {
            sec: item.timestamp(),
            nanosec: item.nanosecond(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct DurationElement {
    sec: i64,
    nanosec: i64,
}

impl From<DurationElement> for Duration {
    fn from(item: DurationElement) -> Self {
        Duration::seconds(item.sec) + Duration::nanoseconds(item.nanosec)
    }
}

impl From<Duration> for DurationElement {
    fn from(item: Duration) -> Self {
        Self {
            sec: item.num_seconds(),
            nanosec: item.num_nanoseconds().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct VectorElement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Vector3<f64>> for VectorElement {
    fn from(item: Vector3<f64>) -> Self {
        Self {
            x: item.x,
            y: item.y,
            z: item.z,
        }
    }
}

impl From<VectorElement> for Vector3<f64> {
    fn from(item: VectorElement) -> Self {
        Vector3::new(item.x, item.y, item.z)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub struct QuaternionElement {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<UnitQuaternion<f64>> for QuaternionElement {
    fn from(item: UnitQuaternion<f64>) -> Self {
        Self {
            x: item.i,
            y: item.j,
            z: item.k,
            w: item.w,
        }
    }
}

impl From<QuaternionElement> for Quaternion<f64> {
    fn from(item: QuaternionElement) -> Self {
        Self::new(item.w, item.x, item.y, item.z)
    }
}

impl From<QuaternionElement> for UnitQuaternion<f64> {
    fn from(item: QuaternionElement) -> Self {
        let quaternion = Quaternion::from(item);
        Self::from_quaternion(quaternion)
    }
}
