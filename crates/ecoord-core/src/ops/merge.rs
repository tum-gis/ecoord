use crate::{FrameId, FrameInfo, TransformId, TransformTree};

use crate::error::Error;

use crate::transform_edge::TransformEdge;
use std::collections::HashMap;

/// Merges a list of transform trees to a single transform tree.
/// Requires unique [TransformId] combinations across the input [TransformTree].
pub fn merge(transform_trees: &[TransformTree]) -> Result<TransformTree, Error> {
    let mut combined_edges: HashMap<TransformId, TransformEdge> = HashMap::new();
    let mut combined_frames: HashMap<FrameId, FrameInfo> = HashMap::new();

    for current_transform_tree in transform_trees {
        current_transform_tree.edges.iter().for_each(|t| {
            combined_edges.insert(t.0.clone(), t.1.clone());
        });

        current_transform_tree.frames.iter().for_each(|t| {
            combined_frames.insert(t.0.clone(), t.1.clone());
        });
    }

    TransformTree::new(
        combined_edges.into_values().collect(),
        combined_frames.into_values().collect(),
    )
}
