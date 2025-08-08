use crate::kitti::error::Error;
use chrono::{DateTime, Utc};
use ecoord_core::{
    ChannelId, ExtrapolationMethod, FrameId, InterpolationMethod, ReferenceFrames, Transform,
    TransformId, TransformInfo,
};
use nalgebra::{Isometry3, matrix};
use std::collections::HashMap;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
struct Record {
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    e: f64,
    f: f64,
    g: f64,
    h: f64,
    i: f64,
    j: f64,
    k: f64,
    l: f64,
}

impl From<Record> for nalgebra::Matrix4<f64> {
    fn from(value: Record) -> Self {
        matrix![value.a, value.b, value.c, value.d;
                value.e, value.f, value.g, value.h;
                value.i, value.j, value.k, value.l;
        0.0, 0.0, 0.0, 1.0]
    }
}

pub fn read_from_csv_file<R: Read>(
    reader: R,
    start_date_time: DateTime<Utc>,
    stop_date_time: DateTime<Utc>,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    world_offset_channel_id: ChannelId,
    world_frame_id: FrameId,
    world_offset: Option<nalgebra::Vector3<f64>>,
) -> Result<ReferenceFrames, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b' ')
        .from_reader(reader);

    let mut records: Vec<nalgebra::Matrix4<f64>> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        records.push(record.into());
    }

    let records_isometries: Vec<Isometry3<f64>> = records
        .into_iter()
        .map(|x| nalgebra::try_convert(x).ok_or(Error::IsometryNotDerivable()))
        .collect::<Result<_, _>>()?;

    let total_duration = stop_date_time - start_date_time;
    let step_duration = total_duration / records_isometries.len() as i32;

    let transforms: Vec<Transform> = records_isometries
        .into_iter()
        .enumerate()
        .map(|(i, isometry)| {
            let timestamp = start_date_time + step_duration * i as i32;
            Transform::new(timestamp, isometry.translation.vector, isometry.rotation)
        })
        .collect();
    let transform_id = TransformId::new(trajectory_frame_id.clone(), trajectory_child_frame_id);

    let mut transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> =
        HashMap::from([((trajectory_channel_id, transform_id.clone()), transforms)]);

    let mut transform_info: HashMap<TransformId, TransformInfo> = HashMap::from([(
        transform_id,
        TransformInfo::new(InterpolationMethod::Linear, ExtrapolationMethod::Constant),
    )]);

    if let Some(world_offset) = world_offset {
        let world_transform_id =
            TransformId::new(world_frame_id.clone(), trajectory_frame_id.clone());
        let world_transform = Transform::new(
            start_date_time,
            world_offset,
            nalgebra::UnitQuaternion::identity(),
        );
        transforms.insert(
            (world_offset_channel_id, world_transform_id.clone()),
            vec![world_transform],
        );

        transform_info.insert(
            world_transform_id,
            TransformInfo::new(InterpolationMethod::Step, ExtrapolationMethod::Constant),
        );
    }

    let reference_frames =
        ReferenceFrames::new(transforms, HashMap::new(), HashMap::new(), transform_info)?;
    Ok(reference_frames)
}
