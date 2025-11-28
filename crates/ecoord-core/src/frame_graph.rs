use crate::Error;
use crate::Error::{InvalidFrameId, MultipleTransformPaths, NoTransformPath};
use crate::frames::FrameId;
use crate::transform::TransformId;
use itertools::Itertools;
use petgraph::data::DataMap;
use petgraph::graph::NodeIndex;
use petgraph::{Directed, Graph, algo};
use std::collections::{HashMap, HashSet};
use std::hash::RandomState;

/// Implements a single transform graph at a single point in time.
///
///
#[derive(Debug, Clone, Default)]
pub(crate) struct FrameGraph {
    graph: Graph<FrameId, ()>,
    frame_id_to_node_index_map: HashMap<FrameId, NodeIndex>,
}

impl FrameGraph {
    pub fn new(transform_ids: HashSet<TransformId>) -> Result<Self, Error> {
        let mut graph = Graph::<FrameId, (), Directed>::new();
        let frame_ids: HashSet<FrameId> = transform_ids
            .iter()
            .flat_map(|t| [t.parent_frame_id.clone(), t.child_frame_id.clone()])
            .collect();
        let frame_id_to_node_index_map: HashMap<FrameId, NodeIndex> = frame_ids
            .into_iter()
            .map(|x| (x.clone(), graph.add_node(x)))
            .collect();

        // remove clone
        for current_transform_id in transform_ids {
            let parent_frame_node_id = frame_id_to_node_index_map
                .get(&current_transform_id.parent_frame_id)
                .expect("must be available");
            let child_frame_node_id = frame_id_to_node_index_map
                .get(&current_transform_id.child_frame_id)
                .expect("must be available");

            graph.add_edge(*parent_frame_node_id, *child_frame_node_id, ());
        }

        let frame_graph = Self {
            graph,
            frame_id_to_node_index_map,
        };

        Ok(frame_graph)
    }

    /// Returns all frame ids that are children of a transform.
    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        self.frame_id_to_node_index_map.keys().cloned().collect()
    }

    pub fn contains_frame_id(&self, frame_id: &FrameId) -> bool {
        self.frame_id_to_node_index_map.contains_key(frame_id)
    }

    pub fn get_frame_id_path(&self, transform_id: &TransformId) -> Result<Vec<FrameId>, Error> {
        let frame_node_index = self
            .frame_id_to_node_index_map
            .get(&transform_id.parent_frame_id)
            .ok_or(InvalidFrameId(transform_id.parent_frame_id.clone()))?;
        let child_frame_node_index = self
            .frame_id_to_node_index_map
            .get(&transform_id.child_frame_id)
            .ok_or(InvalidFrameId(transform_id.child_frame_id.clone()))?;

        let paths = algo::all_simple_paths::<Vec<_>, _, RandomState>(
            &self.graph,
            *frame_node_index,
            *child_frame_node_index,
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

        let chosen_path: &Vec<NodeIndex> =
            paths.first().expect("must have at least one path by now");
        let frame_ids_on_path: Vec<FrameId> = chosen_path
            .iter()
            .map(|idx| {
                self.graph
                    .node_weight(*idx)
                    .expect("node must exist")
                    .clone()
            })
            .collect();

        Ok(frame_ids_on_path)
    }

    pub fn get_transform_id_path(
        &self,
        transform_id: &TransformId,
    ) -> Result<Vec<TransformId>, Error> {
        let transform_id_path = self
            .get_frame_id_path(transform_id)?
            .windows(2)
            .map(|x| TransformId::new(x[0].clone(), x[1].clone()))
            .collect();

        Ok(transform_id_path)
    }

    /// Returns all root nodes (nodes with no incoming edges).
    pub fn root_frames(&self) -> HashSet<FrameId> {
        self.graph
            .node_indices()
            .filter(|node_idx| {
                self.graph
                    .neighbors_directed(*node_idx, petgraph::Direction::Incoming)
                    .count()
                    == 0
            })
            .filter_map(|node_idx| self.graph.node_weight(node_idx).cloned())
            .collect()
    }

    /// Returns all child nodes (nodes with no outgoing edges).
    pub fn child_frames(&self) -> HashSet<FrameId> {
        self.graph
            .node_indices()
            .filter(|node_idx| {
                self.graph
                    .neighbors_directed(*node_idx, petgraph::Direction::Outgoing)
                    .count()
                    == 0
            })
            .filter_map(|node_idx| self.graph.node_weight(node_idx).cloned())
            .collect()
    }
}

#[cfg(test)]
mod test_graph {
    use crate::frame_graph::FrameGraph;
    use crate::{FrameId, TransformId};

    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_basic_interpolation() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::base_link()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let result = frame_graph
            .get_frame_id_path(&(FrameId::map(), "lidar_front_right".into()).into())
            .unwrap();

        assert_eq!(
            result,
            vec![
                FrameId::map(),
                FrameId::base_link(),
                "lidar_front_right".into()
            ]
        );
    }

    #[test]
    fn test_two_direct_root_nodes() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::global(), FrameId::submap()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let result = frame_graph
            .get_frame_id_path(&(FrameId::global(), FrameId::submap()).into())
            .unwrap();

        assert_eq!(result, vec![FrameId::global(), FrameId::submap()]);
    }

    #[test]
    fn test_two_root_nodes() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::submap(), FrameId::base_link()).into(),
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::global(), FrameId::submap()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let result = frame_graph
            .get_frame_id_path(&(FrameId::global(), FrameId::base_link()).into())
            .unwrap();

        assert_eq!(
            result,
            vec![FrameId::global(), FrameId::submap(), FrameId::base_link()]
        );
    }

    #[test]
    fn test_three_level_nodes() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::global(), FrameId::submap()).into(),
            (FrameId::submap(), FrameId::base_link()).into(),
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let result = frame_graph
            .get_frame_id_path(&(FrameId::global(), "lidar_front_right".into()).into())
            .unwrap();

        assert_eq!(
            result,
            vec![
                FrameId::global(),
                FrameId::submap(),
                FrameId::base_link(),
                "lidar_front_right".into()
            ]
        );
    }

    #[test]
    fn test_root_frames_single_root() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::base_link()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let root_nodes = frame_graph.root_frames();

        assert_eq!(root_nodes.len(), 1);
        assert!(root_nodes.contains(&FrameId::map()));
    }

    #[test]
    fn test_root_frames_multiple_disconnected_roots() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::global(), FrameId::base_link()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let root_nodes = frame_graph.root_frames();

        assert_eq!(root_nodes.len(), 2);
        assert!(root_nodes.contains(&FrameId::map()));
        assert!(root_nodes.contains(&FrameId::global()));
    }

    #[test]
    fn test_root_frames_converging_roots() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::global(), FrameId::submap()).into(),
            (FrameId::submap(), FrameId::base_link()).into(),
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let root_nodes = frame_graph.root_frames();

        assert_eq!(root_nodes.len(), 2);
        assert!(root_nodes.contains(&FrameId::global()));
        assert!(root_nodes.contains(&FrameId::map()));
        // submap is not a root node because it has incoming edges
        assert!(!root_nodes.contains(&FrameId::submap()));
    }

    #[test]
    fn test_root_frames_simple_parent_child() {
        let transform_ids: HashSet<TransformId> =
            HashSet::from([(FrameId::map(), FrameId::base_link()).into()]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let root_nodes = frame_graph.root_frames();

        assert_eq!(root_nodes.len(), 1);
        assert!(root_nodes.contains(&FrameId::map()));
        assert!(!root_nodes.contains(&FrameId::base_link()));
    }

    #[test]
    fn test_child_frames_single_child() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::base_link()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let child_nodes = frame_graph.child_frames();

        assert_eq!(child_nodes.len(), 2);
        assert!(child_nodes.contains(&"lidar_front_left".into()));
        assert!(child_nodes.contains(&"lidar_front_right".into()));
    }

    #[test]
    fn test_child_frames_multiple_disconnected_children() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::global(), FrameId::base_link()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let child_nodes = frame_graph.child_frames();

        assert_eq!(child_nodes.len(), 2);
        assert!(child_nodes.contains(&FrameId::submap()));
        assert!(child_nodes.contains(&"lidar_front_right".into()));
    }

    #[test]
    fn test_child_frames_converging_to_single_child() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::global(), FrameId::submap()).into(),
            (FrameId::submap(), FrameId::base_link()).into(),
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let child_nodes = frame_graph.child_frames();

        assert_eq!(child_nodes.len(), 1);
        assert!(child_nodes.contains(&"lidar_front_left".into()));
        // base_link is not a child node because it has outgoing edges
        assert!(!child_nodes.contains(&FrameId::base_link()));
    }

    #[test]
    fn test_child_frames_simple_parent_child() {
        let transform_ids: HashSet<TransformId> =
            HashSet::from([(FrameId::map(), FrameId::base_link()).into()]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let child_nodes = frame_graph.child_frames();

        assert_eq!(child_nodes.len(), 1);
        assert!(child_nodes.contains(&FrameId::base_link()));
        assert!(!child_nodes.contains(&FrameId::map()));
    }

    #[test]
    fn test_child_frames_deep_hierarchy() {
        let transform_ids: HashSet<TransformId> = HashSet::from([
            (FrameId::global(), FrameId::submap()).into(),
            (FrameId::submap(), FrameId::base_link()).into(),
            (FrameId::map(), FrameId::submap()).into(),
            (FrameId::base_link(), "lidar_front_left".into()).into(),
            (FrameId::base_link(), "lidar_front_right".into()).into(),
        ]);

        let frame_graph = FrameGraph::new(transform_ids).unwrap();
        let child_nodes = frame_graph.child_frames();

        assert_eq!(child_nodes.len(), 2);
        assert!(child_nodes.contains(&"lidar_front_left".into()));
        assert!(child_nodes.contains(&"lidar_front_right".into()));
    }
}
