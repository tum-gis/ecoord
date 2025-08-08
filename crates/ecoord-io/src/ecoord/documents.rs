use chrono::{DateTime, TimeZone, Timelike, Utc};
use ecoord_core::{ExtrapolationMethod, InterpolationMethod, TransformId, TransformInfo};
use nalgebra::{Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EcoordDocument {
    pub transforms: Vec<TransformElement>,
    pub frame_info: Vec<FrameInfoElement>,
    pub channel_info: Vec<ChannelInfoElement>,
    pub transform_info: Vec<TransformInfoElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameInfoElement {
    pub id: String,
    pub crs_epsg: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChannelInfoElement {
    pub id: String,
    pub priority: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TransformInfoElement {
    pub frame_id: String,
    pub child_frame_id: String,
    pub interpolation_method: Option<InterpolationMethodElement>,
    pub extrapolation_method: Option<ExtrapolationMethodElement>,
}

impl From<(&TransformId, &TransformInfo)> for TransformInfoElement {
    fn from(item: (&TransformId, &TransformInfo)) -> Self {
        Self {
            frame_id: item.0.frame_id.clone().into(),
            child_frame_id: item.0.child_frame_id.clone().into(),
            interpolation_method: Some(item.1.interpolation_method.into()),
            extrapolation_method: Some(item.1.extrapolation_method.into()),
        }
    }
}

impl From<TransformInfoElement> for (TransformId, TransformInfo) {
    fn from(item: TransformInfoElement) -> Self {
        let transform_id = TransformId::new(item.frame_id.into(), item.child_frame_id.into());
        let transform_info = TransformInfo::new(
            item.interpolation_method
                .map(|x| x.into())
                .unwrap_or_default(),
            item.extrapolation_method
                .map(|x| x.into())
                .unwrap_or_default(),
        );

        (transform_id, transform_info)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationMethodElement {
    Step,
    Linear,
}

impl From<InterpolationMethod> for InterpolationMethodElement {
    fn from(item: InterpolationMethod) -> Self {
        match item {
            InterpolationMethod::Step => Self::Step,
            InterpolationMethod::Linear => Self::Linear,
        }
    }
}

impl From<InterpolationMethodElement> for InterpolationMethod {
    fn from(item: InterpolationMethodElement) -> Self {
        match item {
            InterpolationMethodElement::Step => Self::Step,
            InterpolationMethodElement::Linear => Self::Linear,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExtrapolationMethodElement {
    Constant,
    Linear,
}

impl From<ExtrapolationMethod> for ExtrapolationMethodElement {
    fn from(item: ExtrapolationMethod) -> Self {
        match item {
            ExtrapolationMethod::Constant => Self::Constant,
            ExtrapolationMethod::Linear => Self::Linear,
        }
    }
}

impl From<ExtrapolationMethodElement> for ExtrapolationMethod {
    fn from(item: ExtrapolationMethodElement) -> Self {
        match item {
            ExtrapolationMethodElement::Constant => Self::Constant,
            ExtrapolationMethodElement::Linear => Self::Linear,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TransformElement {
    pub channel_id: String,
    pub frame_id: String,
    pub child_frame_id: String,
    pub timestamp: TimeElement,
    pub translation: VectorElement,
    pub rotation: QuaternionElement,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
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
