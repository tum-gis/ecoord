use crate::error::Error;
use ecoord_core::{
    DynamicTransform, FrameId, FrameInfo, StaticTransform, TimedTransform, Transform,
    TransformEdge, TransformId, TransformTree,
};
use std::collections::HashMap;

use crate::ecoord::documents::TransformTreeSerde;
use chrono::{DateTime, Utc};
use nalgebra::{Isometry3, Quaternion, UnitQuaternion, Vector3};
use std::io::Read;

/// Read a pose from a json file.
///
pub fn read_from_json_file<R: Read>(reader: R) -> Result<TransformTree, Error> {
    let ecoord_document: TransformTreeSerde = serde_json::from_reader(reader)?;

    let edges: Vec<TransformEdge> = ecoord_document
        .edges
        .into_iter()
        .map(|x| {
            let edge: TransformEdge = x.try_into()?;
            Ok(edge)
        })
        .collect::<Result<Vec<TransformEdge>, Error>>()?;

    let frames: Vec<FrameInfo> = ecoord_document
        .frames
        .into_iter()
        .map(|f| f.into())
        .collect();

    let transform_tree = TransformTree::new(edges, frames)?;
    Ok(transform_tree)
}

pub fn read_from_csv_file<R: Read>(reader: R) -> Result<TransformTree, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(reader);

    let records: Vec<CsvRecord> = rdr.deserialize().collect::<Result<Vec<CsvRecord>, _>>()?;
    let records_grouped_by_transform_id: HashMap<TransformId, Vec<CsvRecord>> =
        records.into_iter().fold(HashMap::new(), |mut acc, record| {
            acc.entry(record.transform_id())
                .or_insert_with(Vec::new)
                .push(record);
            acc
        });

    let transform_edges: Vec<TransformEdge> = records_grouped_by_transform_id
        .into_iter()
        .map(|(current_id, records)| derive_transform_edge(current_id, records))
        .collect::<Result<Vec<TransformEdge>, Error>>()?;

    let transform_tree = TransformTree::new(transform_edges, Vec::new())?;
    Ok(transform_tree)
}

fn derive_transform_edge(
    transform_id: TransformId,
    records: Vec<CsvRecord>,
) -> Result<TransformEdge, Error> {
    if records.len() == 1 && records.first().expect("must be there").timestamp().is_ok() {
        let transform: Transform = records.first().expect("must be there").get_transform();
        let static_transform = StaticTransform::new(
            transform_id.parent_frame_id,
            transform_id.child_frame_id,
            transform,
        );
        Ok(TransformEdge::Static(static_transform))
    } else {
        let timed_transforms: Vec<TimedTransform> = records
            .into_iter()
            .map(|x| x.get_timed_transform())
            .collect::<Result<Vec<TimedTransform>, Error>>()?;

        let dynamic_transform = DynamicTransform::new(
            transform_id.parent_frame_id,
            transform_id.child_frame_id,
            None,
            None,
            timed_transforms,
        )?;
        Ok(TransformEdge::Dynamic(dynamic_transform))
    }
}

#[derive(Clone, Debug, serde::Deserialize, PartialEq, PartialOrd)]
struct CsvRecord {
    parent_frame_id: String,
    child_frame_id: String,
    timestamp: Option<f64>,
    timestamp_sec: Option<i64>,
    timestamp_nanosec: Option<u32>,
    translation_x: f64,
    translation_y: f64,
    translation_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    rotation_w: f64,
}

impl CsvRecord {
    pub fn parent_frame_id(&self) -> FrameId {
        self.parent_frame_id.clone().into()
    }

    pub fn child_frame_id(&self) -> FrameId {
        self.child_frame_id.clone().into()
    }

    pub fn transform_id(&self) -> TransformId {
        TransformId::new(self.parent_frame_id(), self.child_frame_id())
    }

    pub fn timestamp(&self) -> Result<Option<DateTime<Utc>>, Error> {
        if self.timestamp.is_some()
            && (self.timestamp_sec.is_some() || self.timestamp_nanosec.is_some())
        {
            return Err(Error::TimestampDefinedTwice());
        }

        if let Some(timestamp_sec) = self.timestamp_sec {
            let timestamp_nanosec = self.timestamp_nanosec.unwrap_or(0);
            let timestamp = DateTime::<Utc>::from_timestamp(timestamp_sec, timestamp_nanosec)
                .ok_or(Error::InvalidTimestamp())?;
            return Ok(Some(timestamp));
        }

        if let Some(timestamp_f64) = self.timestamp {
            let timestamp_sec = timestamp_f64.trunc() as i64;
            let timestamp_nanosec = (timestamp_f64.fract().abs() * 1_000_000_000.0).round() as u32;
            let timestamp = DateTime::<Utc>::from_timestamp(timestamp_sec, timestamp_nanosec)
                .ok_or(Error::InvalidTimestamp())?;
            return Ok(Some(timestamp));
        }

        Ok(None)
    }

    pub fn translation(&self) -> Vector3<f64> {
        Vector3::new(self.translation_x, self.translation_y, self.translation_z)
    }

    pub fn rotation(&self) -> UnitQuaternion<f64> {
        let quaternion = Quaternion::new(
            self.rotation_w,
            self.rotation_x,
            self.rotation_y,
            self.rotation_z,
        );

        UnitQuaternion::from_quaternion(quaternion)
    }

    pub fn get_timed_transform(&self) -> Result<TimedTransform, Error> {
        let timestamp = self.timestamp()?.ok_or(Error::NoTimestamp())?;
        Ok(TimedTransform::new(timestamp, self.get_transform()))
    }

    pub fn get_transform(&self) -> Transform {
        Transform {
            translation: self.translation(),
            rotation: self.rotation(),
        }
    }

    pub fn isometry(&self) -> Isometry3<f64> {
        Isometry3::from_parts(self.translation().into(), self.rotation())
    }
}
