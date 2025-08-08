use crate::ecoord::documents::{
    ChannelInfoElement, EcoordDocument, FrameInfoElement, TransformElement, TransformInfoElement,
};
use crate::error::Error;
use ecoord_core::ReferenceFrames;
use std::io::Write;

pub fn write_to_json_file<W: Write>(
    writer: W,
    pretty_write: bool,
    reference_frames: &ReferenceFrames,
) -> Result<(), Error> {
    let mut transforms: Vec<TransformElement> = vec![];
    for ((current_channel_id, transform_id), current_transforms) in reference_frames.transforms() {
        let mut current_transform_elements: Vec<TransformElement> = current_transforms
            .iter()
            .map(|t| TransformElement {
                channel_id: current_channel_id.clone().into(),
                frame_id: transform_id.frame_id.clone().into(),
                child_frame_id: transform_id.child_frame_id.clone().into(),
                timestamp: t.timestamp.into(),
                translation: t.translation.into(),
                rotation: t.rotation.into(),
            })
            .collect();

        transforms.append(&mut current_transform_elements);
    }

    let frame_info: Vec<FrameInfoElement> = reference_frames
        .frame_info()
        .iter()
        .map(|f| FrameInfoElement {
            id: f.0.clone().into(),
            crs_epsg: f.1.crs_epsg,
        })
        .collect();

    let channel_info: Vec<ChannelInfoElement> = reference_frames
        .channel_info()
        .iter()
        .map(|f| ChannelInfoElement {
            id: f.0.clone().into(),
            priority: f.1.priority,
        })
        .collect();

    let transform_info: Vec<TransformInfoElement> = reference_frames
        .transform_info()
        .iter()
        .map(|f| f.into())
        .collect();

    let frames_document = EcoordDocument {
        transforms,
        frame_info,
        channel_info,
        transform_info,
    };

    if pretty_write {
        serde_json::to_writer_pretty(writer, &frames_document)?;
    } else {
        serde_json::to_writer(writer, &frames_document)?;
    }

    Ok(())
}
