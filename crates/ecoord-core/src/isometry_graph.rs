use crate::Error;
use crate::Error::{InvalidFrameId, MultipleTransformPaths, NoTransformPath, NoTransforms};
use crate::frame_info::FrameId;
use crate::transform::TransformId;
use nalgebra::Isometry3;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph, algo};
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

/// Implements a single transform graph at a single point in time (without parallel channels).
///
///
#[derive(Debug, Clone)]
pub struct IsometryGraph {
    graph: Graph<FrameId, Isometry3<f64>>,
    frame_id_to_node_index_map: HashMap<FrameId, NodeIndex>,
}

impl IsometryGraph {
    pub fn new(isometry_transforms: HashMap<TransformId, Isometry3<f64>>) -> Result<Self, Error> {
        if isometry_transforms.is_empty() {
            return Err(NoTransforms());
        }

        let mut graph = Graph::<FrameId, Isometry3<f64>, Directed>::new();
        let frame_ids: HashSet<FrameId> = isometry_transforms
            .iter()
            .flat_map(|t| [t.0.frame_id.clone(), t.0.child_frame_id.clone()])
            .collect();
        let frame_id_to_node_index_map: HashMap<FrameId, NodeIndex> = frame_ids
            .into_iter()
            .map(|x| (x.clone(), graph.add_node(x)))
            .collect();

        // remove clone
        for (current_transform_id, current_isometry) in isometry_transforms.clone() {
            let frame_node_id = frame_id_to_node_index_map
                .get(&current_transform_id.frame_id)
                .expect("must be available");
            let child_frame_node_id = frame_id_to_node_index_map
                .get(&current_transform_id.child_frame_id)
                .expect("must be available");

            graph.add_edge(*frame_node_id, *child_frame_node_id, current_isometry);
        }

        let isometry_graph = Self {
            graph,
            frame_id_to_node_index_map,
        };
        Ok(isometry_graph)
    }

    /// Returns all frame ids that are children of a transform.
    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        self.frame_id_to_node_index_map.keys().cloned().collect()
    }

    pub fn contains_frame_id(&self, frame_id: &FrameId) -> bool {
        self.frame_id_to_node_index_map.contains_key(frame_id)
    }

    pub fn get_isometry(&self, transform_id: &TransformId) -> Result<Isometry3<f64>, Error> {
        let frame_node_id = self
            .frame_id_to_node_index_map
            .get(&transform_id.frame_id)
            .ok_or(InvalidFrameId(transform_id.frame_id.clone()))?;
        let child_frame_node_id = self
            .frame_id_to_node_index_map
            .get(&transform_id.child_frame_id)
            .ok_or(InvalidFrameId(transform_id.child_frame_id.clone()))?;

        let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(
            &self.graph,
            *frame_node_id,
            *child_frame_node_id,
            0,
            None,
        )
        .collect::<Vec<_>>();

        if paths.is_empty() {
            return Err(NoTransformPath(transform_id.clone()));
        }
        if paths.len() > 1 {
            return Err(MultipleTransformPaths(transform_id.clone()));
        }

        let mut isometry = Isometry3::identity();
        let chosen_path: &Vec<NodeIndex> =
            paths.first().expect("must have at least one path by now");
        for current_node_index in chosen_path.windows(2) {
            let edge_index = self
                .graph
                .find_edge(current_node_index[0], current_node_index[1])
                .expect("edge must exist");

            let edge_weight = self
                .graph
                .edge_weight(edge_index)
                .expect("must have a weight");

            isometry *= edge_weight;
        }

        Ok(isometry)
    }
}

#[cfg(test)]
mod test_graph {
    use crate::isometry_graph::IsometryGraph;
    use crate::{FrameId, TransformId};

    use nalgebra::{Isometry3, Translation3, UnitQuaternion};
    use std::collections::HashMap;

    #[test]
    fn test_basic_interpolation() {
        let mut isometry_transforms: HashMap<TransformId, Isometry3<f64>> = HashMap::new();
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_map"), FrameId::from("base_link")),
            Isometry3::from_parts(Translation3::new(10.0, 0.0, 0.0), UnitQuaternion::default()),
        );

        isometry_transforms.insert(
            TransformId::new(
                FrameId::from("base_link"),
                FrameId::from("lidar_front_left"),
            ),
            Isometry3::from_parts(Translation3::new(20.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(
                FrameId::from("base_link"),
                FrameId::from("lidar_front_right"),
            ),
            Isometry3::from_parts(Translation3::new(40.0, 0.0, 0.0), UnitQuaternion::default()),
        );

        let isometry_graph = IsometryGraph::new(isometry_transforms).unwrap();
        let result = isometry_graph
            .get_isometry(&TransformId::new(
                FrameId::from("slam_map"),
                FrameId::from("lidar_front_right"),
            ))
            .unwrap();

        assert_eq!(result.translation, Translation3::new(50.0, 0.0, 0.0));
    }

    #[test]
    fn test_two_direct_root_nodes() {
        let mut isometry_transforms: HashMap<TransformId, Isometry3<f64>> = HashMap::new();
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_map"), FrameId::from("slam_submap")),
            Isometry3::from_parts(Translation3::new(10.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(FrameId::from("world"), FrameId::from("slam_submap")),
            Isometry3::from_parts(
                Translation3::new(100.0, 0.0, 0.0),
                UnitQuaternion::default(),
            ),
        );

        let isometry_graph = IsometryGraph::new(isometry_transforms).unwrap();
        let result = isometry_graph
            .get_isometry(&TransformId::new(
                FrameId::from("world"),
                FrameId::from("slam_submap"),
            ))
            .unwrap();

        assert_eq!(result.translation, Translation3::new(100.0, 0.0, 0.0));
    }

    #[test]
    fn test_two_root_nodes() {
        let mut isometry_transforms: HashMap<TransformId, Isometry3<f64>> = HashMap::new();
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_submap"), FrameId::from("base_link")),
            Isometry3::from_parts(Translation3::new(3.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_map"), FrameId::from("slam_submap")),
            Isometry3::from_parts(Translation3::new(10.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(FrameId::from("world"), FrameId::from("slam_submap")),
            Isometry3::from_parts(
                Translation3::new(100.0, 0.0, 0.0),
                UnitQuaternion::default(),
            ),
        );

        let isometry_graph = IsometryGraph::new(isometry_transforms).unwrap();
        let result = isometry_graph
            .get_isometry(&TransformId::new(
                FrameId::from("world"),
                FrameId::from("base_link"),
            ))
            .unwrap();

        assert_eq!(result.translation, Translation3::new(103.0, 0.0, 0.0));
    }

    #[test]
    fn test_three_level_nodes() {
        let mut isometry_transforms: HashMap<TransformId, Isometry3<f64>> = HashMap::new();

        isometry_transforms.insert(
            TransformId::new(FrameId::from("world"), FrameId::from("slam_submap")),
            Isometry3::from_parts(
                Translation3::new(100.0, 0.0, 0.0),
                UnitQuaternion::default(),
            ),
        );
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_submap"), FrameId::from("base_link")),
            Isometry3::from_parts(Translation3::new(3.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(FrameId::from("slam_map"), FrameId::from("slam_submap")),
            Isometry3::from_parts(Translation3::new(10.0, 0.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(
                FrameId::from("base_link"),
                FrameId::from("lidar_front_left"),
            ),
            Isometry3::from_parts(Translation3::new(0.0, 5.0, 0.0), UnitQuaternion::default()),
        );
        isometry_transforms.insert(
            TransformId::new(
                FrameId::from("base_link"),
                FrameId::from("lidar_front_right"),
            ),
            Isometry3::from_parts(Translation3::new(0.0, 4.0, 0.0), UnitQuaternion::default()),
        );

        let isometry_graph = IsometryGraph::new(isometry_transforms).unwrap();
        let result = isometry_graph
            .get_isometry(&TransformId::new(
                FrameId::from("world"),
                FrameId::from("lidar_front_right"),
            ))
            .unwrap();

        assert_eq!(result.translation, Translation3::new(103.0, 4.0, 0.0));
    }
}
