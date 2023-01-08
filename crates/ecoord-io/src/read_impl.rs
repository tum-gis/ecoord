use crate::documents::EcoordDocument;
use crate::error::Error;
use ecoord_core::{
    ChannelId, ChannelInfo, FrameId, FrameInfo, InterpolationMethod, ReferenceFrames, Transform,
    TransformId, TransformInfo,
};
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;

/// Read a pose from a json file.
///
pub fn read_from_json_file(document_path: impl AsRef<Path>) -> Result<ReferenceFrames, Error> {
    let file = File::open(document_path)?;
    let ecoord_document: EcoordDocument = serde_json::from_reader(&file)?;

    let mut transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = HashMap::new();
    for current_transform_element in ecoord_document.transforms {
        let current_transform_id = (
            ChannelId::from(current_transform_element.channel_id),
            TransformId::new(
                current_transform_element.frame_id.into(),
                current_transform_element.child_frame_id.into(),
            ),
        );
        let current_transform = Transform::new(
            current_transform_element.timestamp.into(),
            current_transform_element.duration.map(|d| d.into()),
            current_transform_element.translation.into(),
            current_transform_element.rotation.into(),
        );

        transforms
            .entry(current_transform_id)
            .or_default()
            .push(current_transform);
    }

    let frame_info: HashMap<FrameId, FrameInfo> = ecoord_document
        .frame_info
        .iter()
        .map(|f| (f.id.clone().into(), FrameInfo::new(f.crs_epsg)))
        .collect();

    let channel_info: HashMap<ChannelId, ChannelInfo> = ecoord_document
        .channel_info
        .iter()
        .map(|f| (f.id.clone().into(), ChannelInfo::new(f.priority)))
        .collect();

    let transform_info: HashMap<TransformId, TransformInfo> = ecoord_document
        .transform_info
        .iter()
        .map(|f| {
            let interpolation_method: Option<ecoord_core::InterpolationMethod> = f
                .interpolation_method
                .clone()
                .map(|i| InterpolationMethod::from_str(&i).unwrap());
            (
                TransformId::new(f.frame_id.clone().into(), f.child_frame_id.clone().into()),
                TransformInfo::new(interpolation_method),
            )
        })
        .collect();

    let reference_frames =
        ReferenceFrames::new(transforms, frame_info, channel_info, transform_info);
    Ok(reference_frames)
}
