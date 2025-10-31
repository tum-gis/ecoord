use crate::kitti::error::Error;
use chrono::{DateTime, TimeZone, Utc};
use ecoord_core::{
    ChannelId, ExtrapolationMethod, FrameId, InterpolationMethod, ReferenceFrames, Transform,
    TransformId, TransformInfo,
};
use nalgebra::{Isometry3, Quaternion, UnitQuaternion, Vector3};
use std::collections::HashMap;
use std::io::Read;

#[derive(Clone, Copy, Debug, serde::Deserialize, PartialEq, PartialOrd)]
struct Record {
    timestamp_sec: i64,
    timestamp_nanosec: u32,
    translation_x: f64,
    translation_y: f64,
    translation_z: f64,
    rotation_x: f64,
    rotation_y: f64,
    rotation_z: f64,
    rotation_w: f64,
}

impl Record {
    pub fn timestamp(&self) -> DateTime<Utc> {
        DateTime::<Utc>::from_timestamp(self.timestamp_sec, self.timestamp_nanosec)
            .expect("must be valid")
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

    pub fn isometry(&self) -> Isometry3<f64> {
        Isometry3::from_parts(self.translation().into(), self.rotation())
    }
}

pub fn read_from_csv_file<R: Read>(
    reader: R,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
) -> Result<ReferenceFrames, Error> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(b';')
        .from_reader(reader);

    let mut transforms: Vec<Transform> = Vec::new();
    for result in rdr.deserialize() {
        let record: Record = result?;
        let transform = Transform::new(record.timestamp(), record.translation(), record.rotation());
        transforms.push(transform);
    }
    transforms.sort_by_key(|x| x.timestamp);
    let transform_id = TransformId::new(trajectory_frame_id.clone(), trajectory_child_frame_id);
    let start_date_time = transforms.first().expect("should be there").timestamp;

    let transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> =
        HashMap::from([((trajectory_channel_id, transform_id.clone()), transforms)]);

    let transform_info: HashMap<TransformId, TransformInfo> = HashMap::from([(
        transform_id,
        TransformInfo::new(InterpolationMethod::Linear, ExtrapolationMethod::Constant),
    )]);

    let reference_frames =
        ReferenceFrames::new(transforms, HashMap::new(), HashMap::new(), transform_info)?;
    Ok(reference_frames)
}
