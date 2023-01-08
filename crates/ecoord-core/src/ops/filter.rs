use crate::{ChannelId, Transform, TransformId};
use std::collections::{HashMap, HashSet};

pub fn filter_by_channel(
    transforms: &HashMap<(ChannelId, TransformId), Vec<Transform>>,
    channel_ids: &HashSet<ChannelId>,
) -> HashMap<(ChannelId, TransformId), Vec<Transform>> {
    assert!(
        !channel_ids.is_empty(),
        "At least one channel name must be provided for filtering"
    );
    //assert!(channel_names.iter().any(""), "Channel names must not be empty.");

    transforms
        .iter()
        .filter(|((channel_id, _), _)| channel_ids.contains(channel_id))
        .map(|(key, item)| (key.clone(), item.clone()))
        .collect()
}
