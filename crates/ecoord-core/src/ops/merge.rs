use crate::{
    ChannelId, ChannelInfo, FrameId, FrameInfo, ReferenceFrames, Transform, TransformId,
    TransformInfo,
};

use crate::error::Error;

use crate::Error::ChannelTransformCollisions;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

/// Merges a list of reference frame systems to a single reference frame system.
/// Requires unique [ChannelId] and [TransformId] combinations across the input [ReferenceFrames].
pub fn merge(reference_frames: &[ReferenceFrames]) -> Result<ReferenceFrames, Error> {
    let all_combinations: HashSet<&(ChannelId, TransformId)> = reference_frames
        .iter()
        .flat_map(|r| r.transforms.keys())
        .collect();
    for current_combination in all_combinations {
        let number_of_occurrences = reference_frames
            .iter()
            .filter(|r| r.transforms.keys().contains(current_combination))
            .count();
        if number_of_occurrences > 1 {
            return Err(ChannelTransformCollisions {
                channel_id: current_combination.0.clone(),
                transform_id: current_combination.1.clone(),
            });
        }
    }

    let mut all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = HashMap::new();
    let mut all_frame_infos: HashMap<FrameId, FrameInfo> = HashMap::new();
    let mut all_channel_infos: HashMap<ChannelId, ChannelInfo> = HashMap::new();
    let mut all_transform_infos: HashMap<TransformId, TransformInfo> = HashMap::new();

    for current_reference_frame in reference_frames {
        current_reference_frame.transforms.iter().for_each(|t| {
            all_transforms
                .entry(t.0.clone())
                .or_default()
                .append(&mut t.1.clone())
        });

        current_reference_frame.frame_info.iter().for_each(|t| {
            // TODO: error, if different frame infos are available
            all_frame_infos.insert(t.0.clone(), t.1.clone());
        });

        current_reference_frame.channel_info.iter().for_each(|t| {
            // TODO: error, if different channel infos are available
            all_channel_infos.insert(t.0.clone(), t.1.clone());
        });

        current_reference_frame.transform_info.iter().for_each(|t| {
            // TODO: error, if different channel infos are available
            all_transform_infos.insert(t.0.clone(), t.1.clone());
        });
    }

    for current_transform in all_transforms.values_mut() {
        current_transform.sort_by_key(|t| t.timestamp.timestamp_nanos());
    }

    ReferenceFrames::new(
        all_transforms,
        all_frame_infos,
        all_channel_infos,
        all_transform_infos,
    )
}
