use chrono::{DateTime, TimeZone, Timelike, Utc};
use ecoord_core::{ExtrapolationMethod, InterpolationMethod, TransformEdge};
use nalgebra::{Quaternion, UnitQuaternion, Vector3};
use serde::{Deserialize, Serialize};

pub type FrameIdSerde = String;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TransformTreeSerde {
    pub edges: Vec<TransformEdgeSerde>,
    pub frames: Vec<FrameSerde>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TransformEdgeSerde {
    Static(StaticTransformSerde),
    Dynamic(DynamicTransformSerde),
}

impl TryFrom<TransformEdgeSerde> for ecoord_core::TransformEdge {
    type Error = ecoord_core::Error;

    fn try_from(item: TransformEdgeSerde) -> Result<Self, Self::Error> {
        match item {
            TransformEdgeSerde::Static(x) => Ok(Self::Static(x.into())),
            TransformEdgeSerde::Dynamic(x) => Ok(Self::Dynamic(x.try_into()?)),
        }
    }
}

impl From<ecoord_core::TransformEdge> for TransformEdgeSerde {
    fn from(item: ecoord_core::TransformEdge) -> Self {
        match item {
            TransformEdge::Static(x) => Self::Static(x.into()),
            TransformEdge::Dynamic(x) => Self::Dynamic(x.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticTransformSerde {
    pub parent_frame_id: String,
    pub child_frame_id: String,
    pub transform: TransformSerde,
}

impl From<StaticTransformSerde> for ecoord_core::StaticTransform {
    fn from(item: StaticTransformSerde) -> Self {
        Self::new(
            item.parent_frame_id.into(),
            item.child_frame_id.into(),
            item.transform.into(),
        )
    }
}

impl From<ecoord_core::StaticTransform> for StaticTransformSerde {
    fn from(item: ecoord_core::StaticTransform) -> Self {
        Self {
            parent_frame_id: item.parent_frame_id().clone().into(),
            child_frame_id: item.child_frame_id().clone().into(),
            transform: item.transform.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicTransformSerde {
    pub parent_frame_id: String,
    pub child_frame_id: String,
    pub interpolation: Option<InterpolationMethodSerde>,
    pub extrapolation: Option<ExtrapolationMethodSerde>,
    pub samples: Vec<TimedTransformSerde>,
}

impl TryFrom<DynamicTransformSerde> for ecoord_core::DynamicTransform {
    type Error = ecoord_core::Error;

    fn try_from(item: DynamicTransformSerde) -> Result<Self, Self::Error> {
        Self::new(
            item.parent_frame_id.into(),
            item.child_frame_id.into(),
            item.interpolation.map(|x| x.into()),
            item.extrapolation.map(|x| x.into()),
            item.samples.into_iter().map(|x| x.into()).collect(),
        )
    }
}

impl From<ecoord_core::DynamicTransform> for DynamicTransformSerde {
    fn from(item: ecoord_core::DynamicTransform) -> Self {
        Self {
            parent_frame_id: item.parent_frame_id().clone().into(),
            child_frame_id: item.child_frame_id().clone().into(),
            interpolation: item.interpolation.map(|x| x.into()),
            extrapolation: item.extrapolation.map(|x| x.into()),
            samples: item.samples.into_iter().map(|x| x.into()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimedTransformSerde {
    pub timestamp: TimeSerde,
    pub transform: TransformSerde,
}

impl From<TimedTransformSerde> for ecoord_core::TimedTransform {
    fn from(item: TimedTransformSerde) -> Self {
        Self {
            timestamp: item.timestamp.into(),
            transform: item.transform.into(),
        }
    }
}

impl From<ecoord_core::TimedTransform> for TimedTransformSerde {
    fn from(item: ecoord_core::TimedTransform) -> Self {
        Self {
            timestamp: item.timestamp.into(),
            transform: item.transform.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformSerde {
    pub translation: VectorSerde,
    pub rotation: QuaternionSerde,
}

impl From<TransformSerde> for ecoord_core::Transform {
    fn from(item: TransformSerde) -> Self {
        Self {
            translation: item.translation.into(),
            rotation: item.rotation.into(),
        }
    }
}

impl From<ecoord_core::Transform> for TransformSerde {
    fn from(item: ecoord_core::Transform) -> Self {
        Self {
            translation: item.translation.into(),
            rotation: item.rotation.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrameSerde {
    pub id: FrameIdSerde,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub crs_epsg: Option<u32>,
}

impl From<FrameSerde> for ecoord_core::FrameInfo {
    fn from(item: FrameSerde) -> Self {
        Self {
            id: item.id.into(),
            description: item.description,
            crs_epsg: item.crs_epsg,
        }
    }
}

impl From<ecoord_core::FrameInfo> for FrameSerde {
    fn from(item: ecoord_core::FrameInfo) -> Self {
        Self {
            id: item.id.into(),
            description: item.description,
            crs_epsg: item.crs_epsg,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InterpolationMethodSerde {
    Step,
    #[default]
    Linear,
}

impl From<InterpolationMethod> for InterpolationMethodSerde {
    fn from(item: InterpolationMethod) -> Self {
        match item {
            InterpolationMethod::Step => Self::Step,
            InterpolationMethod::Linear => Self::Linear,
        }
    }
}

impl From<InterpolationMethodSerde> for InterpolationMethod {
    fn from(item: InterpolationMethodSerde) -> Self {
        match item {
            InterpolationMethodSerde::Step => Self::Step,
            InterpolationMethodSerde::Linear => Self::Linear,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ExtrapolationMethodSerde {
    #[default]
    Constant,
    Linear,
}

impl From<ExtrapolationMethod> for ExtrapolationMethodSerde {
    fn from(item: ExtrapolationMethod) -> Self {
        match item {
            ExtrapolationMethod::Constant => Self::Constant,
            ExtrapolationMethod::Linear => Self::Linear,
        }
    }
}

impl From<ExtrapolationMethodSerde> for ExtrapolationMethod {
    fn from(item: ExtrapolationMethodSerde) -> Self {
        match item {
            ExtrapolationMethodSerde::Constant => Self::Constant,
            ExtrapolationMethodSerde::Linear => Self::Linear,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct TimeSerde {
    sec: i64,
    nanosec: u32,
}

impl From<TimeSerde> for DateTime<Utc> {
    fn from(item: TimeSerde) -> Self {
        Utc.timestamp_opt(item.sec, item.nanosec).unwrap()
    }
}

impl From<DateTime<Utc>> for TimeSerde {
    fn from(item: DateTime<Utc>) -> Self {
        Self {
            sec: item.timestamp(),
            nanosec: item.nanosecond(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct VectorSerde {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<Vector3<f64>> for VectorSerde {
    fn from(item: Vector3<f64>) -> Self {
        Self {
            x: item.x,
            y: item.y,
            z: item.z,
        }
    }
}

impl From<VectorSerde> for Vector3<f64> {
    fn from(item: VectorSerde) -> Self {
        Vector3::new(item.x, item.y, item.z)
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct QuaternionSerde {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl From<UnitQuaternion<f64>> for QuaternionSerde {
    fn from(item: UnitQuaternion<f64>) -> Self {
        Self {
            x: item.i,
            y: item.j,
            z: item.k,
            w: item.w,
        }
    }
}

impl From<QuaternionSerde> for Quaternion<f64> {
    fn from(item: QuaternionSerde) -> Self {
        Self::new(item.w, item.x, item.y, item.z)
    }
}

impl From<QuaternionSerde> for UnitQuaternion<f64> {
    fn from(item: QuaternionSerde) -> Self {
        let quaternion = Quaternion::from(item);
        Self::from_quaternion(quaternion)
    }
}
