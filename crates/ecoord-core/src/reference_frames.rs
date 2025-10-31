use crate::channel_info::{ChannelId, ChannelInfo};
use crate::error::Error;
use crate::error::Error::{InvalidChannelId, TransformsNotSorted};
use crate::frame_info::{FrameId, FrameInfo};
use crate::isometry_graph::IsometryGraph;
use crate::ops::filter::filter_by_channel;
use crate::transform::TransformId;
use crate::transform_info::TransformInfo;
use crate::utils::transforms_interpolation::inter_and_extrapolate_transforms;

use crate::Error::{InvalidTransformId, NoChannels, NoTransforms};
use crate::{ExtrapolationMethod, InterpolationMethod, Transform};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use nalgebra::Isometry3;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use std::collections::{HashMap, HashSet};
use std::vec;

/// Represents a list of transforms for representing different coordinate frames.
///
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReferenceFrames {
    pub transforms: HashMap<(ChannelId, TransformId), Vec<Transform>>,
    pub frame_info: HashMap<FrameId, FrameInfo>,
    pub channel_info: HashMap<ChannelId, ChannelInfo>,
    pub transform_info: HashMap<TransformId, TransformInfo>,
}

impl ReferenceFrames {
    pub fn new(
        transforms: HashMap<(ChannelId, TransformId), Vec<Transform>>,
        frame_info: HashMap<FrameId, FrameInfo>,
        channel_info: HashMap<ChannelId, ChannelInfo>,
        transform_info: HashMap<TransformId, TransformInfo>,
    ) -> Result<Self, Error> {
        if !transforms.is_empty() {
            // check if all frames are referenced by a transforms
            for frame in frame_info.keys() {
                let contained_in_transforms = transforms.keys().any(|(_, transform_id)| {
                    &transform_id.frame_id == frame || &transform_id.child_frame_id == frame
                });
                assert!(
                    contained_in_transforms,
                    "No transform is referencing child or parent frame: {frame}"
                );
            }

            for (current_id, current_transform) in &transforms {
                if !current_transform.windows(2).all(|t| {
                    t[0].timestamp
                        .timestamp_nanos_opt()
                        .expect("should be defined")
                        < t[1]
                            .timestamp
                            .timestamp_nanos_opt()
                            .expect("should be defined")
                }) {
                    return Err(TransformsNotSorted {
                        channel_id: current_id.0.clone(),
                        transform_id: current_id.1.clone(),
                    });
                }
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

        Ok(Self {
            transforms: sorted_transforms,
            frame_info,
            channel_info,
            transform_info,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.transforms.is_empty()
    }

    pub fn transforms(&self) -> &HashMap<(ChannelId, TransformId), Vec<Transform>> {
        &self.transforms
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
}

impl ReferenceFrames {
    pub fn get_channel_ids(&self) -> HashSet<ChannelId> {
        self.transforms
            .keys()
            .map(|(channel_id, _)| channel_id.clone())
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

    pub fn get_transforms_of_channel(
        &self,
        channel_id: &ChannelId,
        transform_id: &TransformId,
    ) -> Result<&Vec<Transform>, Error> {
        let transforms = self
            .transforms
            .get(&(channel_id.clone(), transform_id.clone()))
            .ok_or(NoTransforms())?;
        Ok(transforms)
    }

    pub fn get_channel_priority(&self, channel_id: &ChannelId) -> Result<i32, Error> {
        if !self.get_channel_ids().contains(channel_id) {
            return Err(InvalidChannelId(channel_id.clone()));
        }

        let priority = self.channel_info.get(channel_id).and_then(|x| x.priority);
        Ok(priority.unwrap_or_default())
    }

    pub fn get_interpolation_method(&self, transform_id: &TransformId) -> InterpolationMethod {
        self.transform_info
            .get(transform_id)
            .map(|x| x.interpolation_method)
            .unwrap_or_default()
    }

    pub fn get_extrapolation_method(&self, transform_id: &TransformId) -> ExtrapolationMethod {
        self.transform_info
            .get(transform_id)
            .map(|x| x.extrapolation_method)
            .unwrap_or_default()
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
}

impl ReferenceFrames {
    pub fn set_interpolation_method(
        &mut self,
        transform_id: TransformId,
        method: InterpolationMethod,
    ) {
        self.transform_info
            .entry(transform_id)
            .or_default()
            .interpolation_method = method;
    }

    pub fn set_extrapolation_method(
        &mut self,
        transform_id: TransformId,
        method: ExtrapolationMethod,
    ) {
        self.transform_info
            .entry(transform_id)
            .or_default()
            .extrapolation_method = method;
    }

    pub fn add_transforms(
        &mut self,
        channel_id: ChannelId,
        transform_id: TransformId,
        transforms: Vec<Transform>,
        channel_info: Option<ChannelInfo>,
        transform_info: Option<TransformInfo>,
    ) -> Result<(), Error> {
        if transforms.is_empty() {
            return Err(NoTransforms());
        }

        self.transforms
            .insert((channel_id.clone(), transform_id.clone()), transforms);
        if let Some(channel_info) = channel_info {
            self.channel_info.insert(channel_id, channel_info);
        }
        if let Some(transform_info) = transform_info {
            self.transform_info.insert(transform_id, transform_info);
        }
        Ok(())
    }

    // Extend transforms into already existing
    pub fn extend_transforms(
        &mut self,
        channel_id: ChannelId,
        transform_id: TransformId,
        transforms: Vec<Transform>,
    ) -> Result<(), Error> {
        if transforms.is_empty() {
            return Err(NoTransforms());
        }
        let key = (channel_id, transform_id);

        let mut combined_transforms: Vec<Transform> =
            self.transforms.get(&key).map_or(Vec::new(), |x| x.clone());
        combined_transforms.extend(transforms);

        combined_transforms.sort_by_key(|t| t.timestamp);

        self.transforms.insert(key, combined_transforms);

        Ok(())
    }
}

impl ReferenceFrames {
    /// Filter by channel ids
    pub fn filter_by_channel_ids(
        &self,
        channel_ids: &[ChannelId],
    ) -> Result<ReferenceFrames, Error> {
        let all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = self
            .transforms
            .par_iter()
            .filter(|((channel_id, _), _)| channel_ids.contains(channel_id))
            .map(|((channel_id, transform_id), transforms)| {
                (
                    (channel_id.clone(), transform_id.clone()),
                    transforms.clone(),
                )
            })
            .collect();

        let selected_transform_ids: HashSet<TransformId> =
            all_transforms.keys().map(|(_, t)| t.clone()).collect();
        let selected_frame_ids: HashSet<FrameId> = selected_transform_ids
            .par_iter()
            .flat_map(|t| [t.frame_id.clone(), t.child_frame_id.clone()])
            .collect();

        let all_frame_info: HashMap<FrameId, FrameInfo> = self
            .frame_info
            .par_iter()
            .filter(|(i, _)| selected_frame_ids.contains(i))
            .map(|(i, f)| (i.clone(), f.clone()))
            .collect();
        let all_channel_info: HashMap<ChannelId, ChannelInfo> = self
            .channel_info
            .par_iter()
            .filter(|(channel_id, _)| channel_ids.contains(channel_id))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();
        let all_transform_info: HashMap<TransformId, TransformInfo> = self
            .transform_info
            .par_iter()
            .filter(|(i, _)| selected_transform_ids.contains(i))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();

        let reference_frame = ReferenceFrames::new(
            all_transforms,
            all_frame_info,
            all_channel_info,
            all_transform_info,
        )?;
        Ok(reference_frame)
    }

    pub fn filter_by_transform_ids(
        &self,
        transform_ids: &[TransformId],
    ) -> Result<ReferenceFrames, Error> {
        let all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = self
            .transforms
            .par_iter()
            .filter(|((_, transform_id), _)| transform_ids.contains(transform_id))
            .map(|((channel_id, transform_id), transforms)| {
                (
                    (channel_id.clone(), transform_id.clone()),
                    transforms.clone(),
                )
            })
            .collect();

        let selected_channel_ids: HashSet<ChannelId> =
            all_transforms.keys().map(|(c, _)| c.clone()).collect();
        let selected_transform_ids: HashSet<TransformId> =
            all_transforms.keys().map(|(_, t)| t.clone()).collect();
        let selected_frame_ids: HashSet<FrameId> = selected_transform_ids
            .par_iter()
            .flat_map(|t| [t.frame_id.clone(), t.child_frame_id.clone()])
            .collect();

        let all_frame_info: HashMap<FrameId, FrameInfo> = self
            .frame_info
            .par_iter()
            .filter(|(i, _)| selected_frame_ids.contains(i))
            .map(|(i, f)| (i.clone(), f.clone()))
            .collect();
        let all_channel_info: HashMap<ChannelId, ChannelInfo> = self
            .channel_info
            .par_iter()
            .filter(|(channel_id, _)| selected_channel_ids.contains(channel_id))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();
        let all_transform_info: HashMap<TransformId, TransformInfo> = self
            .transform_info
            .par_iter()
            .filter(|(i, _)| selected_transform_ids.contains(i))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();

        let reference_frame = ReferenceFrames::new(
            all_transforms,
            all_frame_info,
            all_channel_info,
            all_transform_info,
        )?;
        Ok(reference_frame)
    }

    pub fn filter_by_time_interval(
        &self,
        start_time: &Option<DateTime<Utc>>,
        stop_time: &Option<DateTime<Utc>>,
    ) -> Result<ReferenceFrames, Error> {
        if start_time.is_none() && stop_time.is_none() {
            return Ok(self.clone());
        }

        let all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = self
            .transforms
            .par_iter()
            .flat_map(|((channel_id, transform_id), transforms)| {
                let filtered_transforms: Vec<Transform> = transforms
                    .iter()
                    .filter(|t| start_time.is_none_or(|x| x <= t.timestamp))
                    .filter(|t| stop_time.is_none_or(|x| t.timestamp < x))
                    .cloned()
                    .collect();
                if filtered_transforms.is_empty() {
                    return None;
                }

                let key = (channel_id.clone(), transform_id.clone());
                Some((key, filtered_transforms))
            })
            .collect();

        let selected_channel_ids: HashSet<ChannelId> =
            all_transforms.keys().map(|(c, _)| c.clone()).collect();
        let selected_transform_ids: HashSet<TransformId> =
            all_transforms.keys().map(|(_, t)| t.clone()).collect();
        let selected_frame_ids: HashSet<FrameId> = selected_transform_ids
            .par_iter()
            .flat_map(|t| [t.frame_id.clone(), t.child_frame_id.clone()])
            .collect();

        let all_frame_info: HashMap<FrameId, FrameInfo> = self
            .frame_info
            .par_iter()
            .filter(|(i, _)| selected_frame_ids.contains(i))
            .map(|(i, f)| (i.clone(), f.clone()))
            .collect();
        let all_channel_info: HashMap<ChannelId, ChannelInfo> = self
            .channel_info
            .par_iter()
            .filter(|(channel_id, _)| selected_channel_ids.contains(channel_id))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();
        let all_transform_info: HashMap<TransformId, TransformInfo> = self
            .transform_info
            .par_iter()
            .filter(|(i, _)| selected_transform_ids.contains(i))
            .map(|(i, c)| (i.clone(), c.clone()))
            .collect();

        let reference_frame = ReferenceFrames::new(
            all_transforms,
            all_frame_info,
            all_channel_info,
            all_transform_info,
        )?;
        Ok(reference_frame)
    }
}

impl ReferenceFrames {
    pub fn get_timed_subset(&self, timestamp: &DateTime<Utc>) -> Result<ReferenceFrames, Error> {
        let all_transforms: HashMap<(ChannelId, TransformId), Vec<Transform>> = self
            .transforms
            .iter()
            .map(|((channel_id, transform_id), transforms)| {
                let isometry = inter_and_extrapolate_transforms(
                    transforms,
                    &Some(*timestamp),
                    self.get_interpolation_method(transform_id),
                    self.get_extrapolation_method(transform_id),
                );

                isometry.map(|i| {
                    (
                        (channel_id.clone(), transform_id.clone()),
                        vec![Transform::from(*timestamp, i)],
                    )
                })
            })
            .collect::<Result<HashMap<(ChannelId, TransformId), Vec<Transform>>, _>>()?;

        let all_transform_info = self
            .transform_info
            .keys()
            .map(|k| {
                (
                    k.clone(),
                    TransformInfo::new(InterpolationMethod::Step, ExtrapolationMethod::default()),
                )
            })
            .collect();

        let reference_frame = ReferenceFrames::new(
            all_transforms,
            self.frame_info.clone(),
            self.channel_info.clone(),
            all_transform_info,
        )?;
        Ok(reference_frame)
    }

    /// Returns the transforms valid at a specific timestamp.
    pub fn get_valid_transform(
        &self,
        channel_id: &ChannelId,
        transform_id: &TransformId,
        timestamp: &Option<DateTime<Utc>>,
    ) -> Result<Vec<&Transform>, Error> {
        if !self.contains_channel(channel_id) {
            return Err(InvalidChannelId(channel_id.clone()));
        }

        let all_transforms: Vec<&Transform> = self
            .transforms
            .get(&(channel_id.clone(), transform_id.clone()))
            .ok_or_else(|| InvalidTransformId(channel_id.clone(), transform_id.clone()))?
            .iter()
            .collect();

        if timestamp.is_none() {
            return Ok(all_transforms);
        }
        let timestamp = timestamp.unwrap();

        let mut time_based_filtered_transforms: Vec<&Transform> = all_transforms
            .clone()
            .windows(2)
            .filter(|t| {
                t[0].timestamp.timestamp_nanos_opt().unwrap()
                    <= timestamp.timestamp_nanos_opt().unwrap()
                    && timestamp.timestamp_nanos_opt().unwrap()
                        < t[1].timestamp.timestamp_nanos_opt().unwrap()
            })
            /*.filter(|t| {
                t[0].duration
                    .map_or(false, |d| timestamp <= t[0].timestamp + d)
                    || timestamp.timestamp_nanos() < t[1].timestamp.timestamp_nanos()
            })*/
            .map(|t| t[0])
            .collect();

        if all_transforms.last().unwrap().timestamp <= timestamp {
            time_based_filtered_transforms.push(all_transforms.last().unwrap());
        }

        Ok(time_based_filtered_transforms)
    }

    /// Derive a concrete transform graph for a specific timestamp and selected channels.
    ///
    /// * `selected_channel_ids` - Selected channels for building the transform graph.
    /// * `selected_timestamp` - Timestamp to choose for interpolating time-dependent transforms.
    pub fn derive_transform_graph(
        &self,
        selected_channel_ids: &Option<HashSet<ChannelId>>,
        selected_timestamp: &Option<DateTime<Utc>>,
    ) -> Result<IsometryGraph, Error> {
        if let Some(channel_names) = &selected_channel_ids
            && channel_names.is_empty()
        {
            return Err(NoChannels());
        }

        let mut selected_isometries: HashMap<TransformId, Isometry3<f64>> = HashMap::new();
        let selected_transforms: &HashMap<(ChannelId, TransformId), Vec<Transform>> =
            match &selected_channel_ids {
                Some(channel_names) => &filter_by_channel(&self.transforms, channel_names),
                None => &self.transforms,
            };

        let mut prioritized_selected_transforms: HashMap<&TransformId, &Vec<Transform>> =
            HashMap::new();
        for (_, group) in &selected_transforms
            .iter()
            .sorted_by_key(|k| &k.0.1)
            .chunk_by(|k| k.0.1.clone())
        {
            let highest_priority = group
                .into_iter()
                .max_by_key(|k| {
                    self.get_channel_priority(&k.0.0)
                        .expect("channel should exist")
                })
                .unwrap();

            prioritized_selected_transforms.insert(&highest_priority.0.1, highest_priority.1);
        }

        for (current_transform_id, current_transforms) in prioritized_selected_transforms {
            let interpolated_transform = inter_and_extrapolate_transforms(
                current_transforms,
                selected_timestamp,
                self.get_interpolation_method(current_transform_id),
                self.get_extrapolation_method(current_transform_id),
            )?;
            selected_isometries.insert(current_transform_id.clone(), interpolated_transform);
        }

        IsometryGraph::new(selected_isometries)
    }
}
