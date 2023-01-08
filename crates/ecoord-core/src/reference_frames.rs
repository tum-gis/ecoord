use crate::channel_info::{ChannelId, ChannelInfo};
use crate::frame_info::{FrameId, FrameInfo};
use crate::isometry_graph::IsometryGraph;
use crate::ops::filter::filter_by_channel;
use crate::transform::TransformId;
use crate::transform_info::TransformInfo;
use crate::transforms_interpolation::interpolate_transforms;
use crate::{InterpolationMethod, Transform};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use nalgebra::Isometry3;
use std::collections::{HashMap, HashSet};

/// Represents a list of transforms for representing different coordinate frames.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReferenceFrames {
    pub(crate) transforms: HashMap<(ChannelId, TransformId), Vec<Transform>>,
    pub(crate) frame_info: HashMap<FrameId, FrameInfo>,
    pub(crate) channel_info: HashMap<ChannelId, ChannelInfo>,
    pub(crate) transform_info: HashMap<TransformId, TransformInfo>,
}

impl ReferenceFrames {
    pub fn new(
        transforms: HashMap<(ChannelId, TransformId), Vec<Transform>>,
        frame_info: HashMap<FrameId, FrameInfo>,
        channel_info: HashMap<ChannelId, ChannelInfo>,
        transform_info: HashMap<TransformId, TransformInfo>,
    ) -> Self {
        if !transforms.is_empty() {
            // check if all frames are referenced by a transforms
            for frame in frame_info.keys() {
                let contained_in_transforms = transforms.keys().any(|(_, transform_id)| {
                    &transform_id.frame_id == frame || &transform_id.child_frame_id == frame
                });
                assert!(
                    contained_in_transforms,
                    "No transform is referencing child or parent frame: {}",
                    frame
                );
            }
        }

        // sorting the transform vectors by time
        let mut sorted_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> =
            HashMap::new();
        for (current_key, current_transforms) in transforms {
            let mut current_sorted_transforms = current_transforms.clone();
            current_sorted_transforms.sort_by_key(|t| t.timestamp);

            sorted_transforms.insert(current_key, current_sorted_transforms);
        }

        //.all(|transforms| transforms.sort_by_key(|t| t.timestamp));

        /*for transform in transforms.iter() {
            assert!(
                frame_info.keys().contains(&transform.frame_id),
                "Frame with id '{}' is missing",
                transform.frame_id
            );
            assert!(
                frame_info.keys().contains(&transform.child_frame_id),
                "Frame with id '{}' is missing",
                transform.child_frame_id
            );
        }*/
        //for (a, b) in transform.iter().tuple_windows() {
        //    assert_eq!(a.child_frame_id, b.parent_frame_id, "Child frame '{}' does not fit to parent frame '{}' of the following transform", a.child_frame_id, b.parent_frame_id);
        //}

        Self {
            transforms: sorted_transforms,
            frame_info,
            channel_info,
            transform_info,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    pub fn frame_info(&self) -> &HashMap<FrameId, FrameInfo> {
        &self.frame_info
    }

    pub fn channel_info(&self) -> &HashMap<ChannelId, ChannelInfo> {
        &self.channel_info
    }

    pub fn transform_info(&self) -> &HashMap<TransformId, TransformInfo> {
        &self.transform_info
    }

    pub fn set_interpolation_method(
        &mut self,
        transform_id: TransformId,
        method: Option<InterpolationMethod>,
    ) {
        self.transform_info
            .entry(transform_id)
            .or_default()
            .interpolation_method = method;
    }

    pub fn transforms(&self) -> &HashMap<(ChannelId, TransformId), Vec<Transform>> {
        &self.transforms
    }

    pub fn get_channel_ids(&self) -> HashSet<ChannelId> {
        self.transforms
            .keys()
            .map(|x| x.0.clone())
            .into_iter()
            .collect()
    }

    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        self.transforms
            .keys()
            .fold(Vec::<FrameId>::new(), |mut acc, x| {
                acc.push(x.1.frame_id.clone());
                acc.push(x.1.child_frame_id.clone());
                acc
            })
            .into_iter()
            .collect()
    }

    pub fn get_channel_names(&self) -> HashSet<ChannelId> {
        self.transforms
            .keys()
            .map(|(channel_id, _)| channel_id.clone())
            .collect()
    }

    pub fn get_channel_priority(&self, channel_id: &ChannelId) -> i32 {
        assert!(self.contains_channel(channel_id));
        self.channel_info
            .get(channel_id)
            .and_then(|x| x.priority)
            .unwrap_or(0)
    }

    pub fn contains_channel(&self, channel_id: &ChannelId) -> bool {
        self.get_channel_ids().iter().any(|x| x == channel_id)
    }

    pub fn contains_frame(&self, frame_id: &FrameId) -> bool {
        self.get_frame_ids().iter().any(|f| f == frame_id)
    }

    pub fn contains_transform(&self, channel_id: &ChannelId, transform_id: &TransformId) -> bool {
        self.transforms
            .keys()
            .any(|(current_channel_id, current_transform_id)| {
                current_channel_id == channel_id && current_transform_id == transform_id
            })
    }

    pub fn add_transform(
        &mut self,
        channel_id: ChannelId,
        transform_id: TransformId,
        transforms: Vec<Transform>,
        channel_info: Option<ChannelInfo>,
        transform_info: Option<TransformInfo>,
    ) {
        self.transforms
            .insert((channel_id.clone(), transform_id.clone()), transforms);
        if let Some(channel_info) = channel_info {
            self.channel_info.insert(channel_id, channel_info);
        }
        if let Some(transform_info) = transform_info {
            self.transform_info.insert(transform_id, transform_info);
        }
    }

    pub fn get_interpolation_method(
        &self,
        transform_id: &TransformId,
    ) -> Option<InterpolationMethod> {
        self.transform_info
            .get(transform_id)
            .and_then(|o| o.interpolation_method)
    }

    /// Derive a concrete transform graph for a specific timestamp and selected channels.
    ///
    /// * `selected_channel_ids` - Selected channels for building the transform graph.
    /// * `selected_timestamp` - Timestamp to choose for interpolating time-dependent transforms.
    pub fn derive_transform_graph(
        &self,
        selected_channel_ids: &Option<HashSet<ChannelId>>,
        selected_timestamp: &Option<DateTime<Utc>>,
    ) -> IsometryGraph {
        if let Some(channel_names) = &selected_channel_ids {
            assert!(!channel_names.is_empty(), "Channel names must not be empty");
        }

        let mut selected_isometries: HashMap<TransformId, Isometry3<f64>> = HashMap::new();
        //if let Some(selected_channel_ids) = selected_channel_ids {
        //    let a: HashMap<&(ChannelId, TransformId), &Transform> = self.transforms.iter().filter(|&(k, v)| selected_channel_ids.contains(&k.0)).collect();
        //    println!("x is None")
        //}

        let selected_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> =
            match &selected_channel_ids {
                Some(channel_names) => filter_by_channel(&self.transforms, channel_names),
                None => self.transforms.clone(),
            };

        let mut prioritized_selected_transforms: HashMap<TransformId, Vec<Transform>> =
            HashMap::new();
        for (_, group) in &selected_transforms.iter().group_by(|k| k.0 .1.clone()) {
            //group.into_iter().for_each(|x| println!("{}, {}", x.0.0, self.get_channel_priority(&x.0.0)));
            let highest_priority = group
                .into_iter()
                .max_by_key(|k| self.get_channel_priority(&k.0 .0))
                .unwrap();
            //println!("{}", highest_priority.0 .0);
            prioritized_selected_transforms
                .insert(highest_priority.0 .1.clone(), highest_priority.1.clone());
        }
        //let test = selected_transforms.iter().group_by(|k| k.1);

        for (current_transform_id, current_transforms) in prioritized_selected_transforms {
            // let key = (current_channel_id.clone(), current_transform_id.clone());
            let interpolation_method = self
                .get_interpolation_method(&current_transform_id)
                .unwrap_or_default();
            let interpolated_transform = interpolate_transforms(
                &current_transforms,
                selected_timestamp,
                interpolation_method,
            );
            selected_isometries.insert(current_transform_id.clone(), interpolated_transform);

            //   .entry(current_transform_id.clone())
            //    .insert_entry(Vec::new)
            //    .push(interpolated_transform);

            /*
            #[cfg(debug_assertions)]
            {
                let min_max_result = current_transforms.iter().minmax_by_key(|t| t.timestamp);
                match min_max_result {
                    MinMaxResult::NoElements => {
                        println!(
                            "transform line frame_id={} child_frame_id={}: no elements",
                            &current_transform_id.frame_id, &current_transform_id.child_frame_id
                        );
                    }
                    MinMaxResult::OneElement(e) => {
                        println!(
                            "transform frame_id={} child_frame_id={}: one element={}",
                            &current_transform_id.frame_id,
                            &current_transform_id.child_frame_id,
                            e.timestamp
                        );
                    }
                    MinMaxResult::MinMax(min, max) => {
                        println!(
                            "transform frame_id={} child_frame_id={}: min={}, max={}",
                            &current_transform_id.frame_id,
                            &current_transform_id.child_frame_id,
                            min.timestamp,
                            max.timestamp
                        );
                    }
                }
            }
            */
        }

        IsometryGraph::new(selected_isometries)
    }
    //
    // pub fn get_start_timestamp(
    //     &self,
    //     selected_channel_ids: Option<&HashSet<ChannelId>>,
    // ) -> DateTime<Utc> {
    //     assert!(selected_channel_ids.is_none(), "Not supported yet.");
    //     self.transforms
    //         .iter()
    //         .flat_map(|t| t.1)
    //         .map(|t| t.timestamp)
    //         .min()
    //         .unwrap()
    // }
}
