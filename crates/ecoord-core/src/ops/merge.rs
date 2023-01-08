use crate::{
    ChannelId, ChannelInfo, FrameId, FrameInfo, ReferenceFrames, Transform, TransformId,
    TransformInfo,
};
use std::collections::HashMap;

pub fn merge(reference_frames: &Vec<ReferenceFrames>) -> ReferenceFrames {
    let mut all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = HashMap::new();
    reference_frames
        .iter()
        .for_each(|t| all_transforms.extend(t.transforms.clone()));

    let mut all_frame_infos: HashMap<FrameId, FrameInfo> = HashMap::new();
    reference_frames
        .iter()
        .for_each(|t| all_frame_infos.extend(t.frame_info.clone()));

    let mut all_channel_infos: HashMap<ChannelId, ChannelInfo> = HashMap::new();
    reference_frames
        .iter()
        .for_each(|t| all_channel_infos.extend(t.channel_info.clone()));

    let mut all_transform_infos: HashMap<TransformId, TransformInfo> = HashMap::new();
    reference_frames
        .iter()
        .for_each(|t| all_transform_infos.extend(t.transform_info.clone()));

    ReferenceFrames::new(
        all_transforms,
        all_frame_infos,
        all_channel_infos,
        all_transform_infos,
    )
}
