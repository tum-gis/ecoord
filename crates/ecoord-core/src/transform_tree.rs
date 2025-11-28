use crate::error::Error;
use crate::frames::{FrameId, FrameInfo};
use crate::transform::TransformId;

use crate::Error::ContainsDynamicTransform;
use crate::frame_graph::FrameGraph;
use crate::transform_edge::TransformEdge;
use crate::{DynamicTransform, StaticTransform, TimedTransform, Transform};
use chrono::{DateTime, Utc};
use nalgebra::Isometry3;
use rayon::iter::ParallelIterator;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct TransformTree {
    pub edges: HashMap<TransformId, TransformEdge>,
    pub frames: HashMap<FrameId, FrameInfo>,
    frame_graph: FrameGraph,
}

impl TransformTree {
    pub fn new(edges: Vec<TransformEdge>, frames: Vec<FrameInfo>) -> Result<Self, Error> {
        let edges: HashMap<TransformId, TransformEdge> = edges
            .into_iter()
            .map(|x| (x.transform_id(), x))
            .collect::<HashMap<_, _>>();
        let mut frames: HashMap<FrameId, FrameInfo> = frames
            .into_iter()
            .map(|x| (x.id.clone(), x))
            .collect::<HashMap<_, _>>();

        for transform_id in edges.keys() {
            if !frames.contains_key(&transform_id.parent_frame_id) {
                frames.insert(
                    transform_id.parent_frame_id.clone(),
                    FrameInfo::new(
                        transform_id.parent_frame_id.clone(),
                        Default::default(),
                        Default::default(),
                    ),
                );
            }
            if !frames.contains_key(&transform_id.child_frame_id) {
                frames.insert(
                    transform_id.child_frame_id.clone(),
                    FrameInfo::new(
                        transform_id.child_frame_id.clone(),
                        Default::default(),
                        Default::default(),
                    ),
                );
            }
        }

        let frame_graph = FrameGraph::new(edges.keys().cloned().collect())?;

        Ok(Self {
            edges,
            frames,
            frame_graph,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    pub fn edges(&self) -> &HashMap<TransformId, TransformEdge> {
        &self.edges
    }

    pub fn frames(&self) -> &HashMap<FrameId, FrameInfo> {
        &self.frames
    }

    pub fn insert_edge(&mut self, edge: TransformEdge) {
        if !self.frames.contains_key(edge.parent_frame_id()) {
            let frame = FrameInfo::new(
                edge.parent_frame_id().clone(),
                Default::default(),
                Default::default(),
            );
            self.frames.insert(edge.parent_frame_id().clone(), frame);
        }
        if !self.frames.contains_key(edge.child_frame_id()) {
            let frame = FrameInfo::new(
                edge.child_frame_id().clone(),
                Default::default(),
                Default::default(),
            );
            self.frames.insert(edge.child_frame_id().clone(), frame);
        }
        self.edges.insert(edge.transform_id().clone(), edge);
    }

    pub fn root_frames(&self) -> HashSet<FrameId> {
        self.frame_graph.root_frames()
    }

    pub fn child_frames(&self) -> HashSet<FrameId> {
        self.frame_graph.child_frames()
    }
}

impl TransformTree {
    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        self.frames.keys().cloned().collect()
    }

    pub fn contains_frame(&self, frame_id: &FrameId) -> bool {
        self.frames.contains_key(frame_id)
    }

    pub fn contains_transform(&self, transform_id: &TransformId) -> bool {
        self.edges.contains_key(transform_id)
    }

    pub fn remove_transform(&mut self, transform_id: &TransformId) {
        self.edges.remove(transform_id);
    }
}

impl TransformTree {
    pub fn static_snapshot_at(&self, timestamp: DateTime<Utc>) -> Result<TransformTree, Error> {
        let transform_edges: Vec<TransformEdge> = self
            .edges
            .values()
            .map(|x| {
                let transform = x.at_time(timestamp);

                let static_transform = StaticTransform::new(
                    x.parent_frame_id().clone(),
                    x.child_frame_id().clone(),
                    transform,
                );
                TransformEdge::Static(static_transform)
            })
            .collect();

        TransformTree::new(transform_edges, self.frames.values().cloned().collect())
    }

    pub fn get_transform_at_time(
        &self,
        transform_id: &TransformId,
        timestamp: DateTime<Utc>,
    ) -> Result<Transform, Error> {
        let transform_id_path = self.frame_graph.get_transform_id_path(transform_id)?;
        let transforms: Vec<Transform> = transform_id_path
            .into_iter()
            .map(|x| self.edges.get(&x).expect("must exist").at_time(timestamp))
            .collect();

        let isometry: Isometry3<f64> =
            transforms
                .into_iter()
                .fold(Isometry3::identity(), |acc, t| {
                    let iso: Isometry3<f64> = t.isometry();
                    acc * iso
                });

        Ok(Transform::from(isometry))
    }

    pub fn get_static_transform(&self, transform_id: &TransformId) -> Result<Transform, Error> {
        let transform_id_path = self.frame_graph.get_transform_id_path(transform_id)?;
        let transforms: Vec<Transform> = transform_id_path
            .into_iter()
            .map(|x| match self.edges.get(&x).expect("must exist") {
                TransformEdge::Static(x) => Ok(x.transform),
                TransformEdge::Dynamic(x) => Err(ContainsDynamicTransform()),
            })
            .collect::<Result<Vec<Transform>, Error>>()?;

        let isometry: Isometry3<f64> =
            transforms
                .into_iter()
                .fold(Isometry3::identity(), |acc, t| {
                    let iso: Isometry3<f64> = t.isometry();
                    acc * iso
                });

        Ok(Transform::from(isometry))
    }

    /// Computes transforms at all dynamic sample timestamps along a transform path.
    ///
    /// This method identifies all dynamic transforms in the path from the parent frame to the
    /// child frame specified by `transform_id`, collects all their sample timestamps, and
    /// computes the complete transform at each of those timestamps.
    pub fn compute_timed_transforms_for_all_samples(
        &self,
        transform_id: &TransformId,
    ) -> Result<Vec<TimedTransform>, Error> {
        let transform_id_path = self.frame_graph.get_transform_id_path(transform_id)?;
        let dynamic_transforms: Vec<&DynamicTransform> = transform_id_path
            .into_iter()
            .filter_map(|x| match self.edges.get(&x).expect("must exist") {
                TransformEdge::Dynamic(dynamic) => Some(dynamic),
                TransformEdge::Static(_) => None,
            })
            .collect();

        let all_sample_timestamps: Vec<DateTime<Utc>> = dynamic_transforms
            .into_iter()
            .flat_map(|x| x.sample_timestamps())
            .collect();

        let timed_transforms = all_sample_timestamps
            .into_iter()
            .map(|x| {
                let timed_transform =
                    TimedTransform::new(x, self.get_transform_at_time(transform_id, x)?);

                Ok(timed_transform)
            })
            .collect::<Result<Vec<TimedTransform>, Error>>()?;

        Ok(timed_transforms)
    }

    /// Checks if a transform path contains only static transforms.
    ///
    /// This method determines whether the entire path from the parent frame to the
    /// child frame specified by `transform_id` consists exclusively of static transforms,
    /// with no dynamic (time-varying) transforms.
    pub fn is_transform_path_static(&self, transform_id: &TransformId) -> Result<bool, Error> {
        let transform_id_path = self.frame_graph.get_transform_id_path(transform_id)?;

        let is_static =
            transform_id_path
                .into_iter()
                .all(|x| match self.edges.get(&x).expect("must exist") {
                    TransformEdge::Static(_) => true,
                    TransformEdge::Dynamic(_) => false,
                });

        Ok(is_static)
    }
}
