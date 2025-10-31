use crate::error::Error;
use ecoord_core::{
    ChannelId, ChannelInfo, FrameId, FrameInfo, ReferenceFrames, Transform, TransformId,
    TransformInfo,
};
use std::collections::HashMap;

use crate::ecoord::documents::EcoordDocument;
use std::io::{BufReader, Read};

/// Read a pose from a json file.
///
pub fn read_from_json_file<R: Read>(reader: R) -> Result<ReferenceFrames, Error> {
    let buffered_reader = BufReader::new(reader);
    let ecoord_document: EcoordDocument = serde_json::from_reader(buffered_reader)?;

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
        .into_iter()
        .map(|f| f.into())
        .collect();

    let reference_frames =
        ReferenceFrames::new(transforms, frame_info, channel_info, transform_info)?;
    Ok(reference_frames)
}
