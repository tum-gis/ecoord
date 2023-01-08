use crate::frame_info::FrameId;
use crate::transform::TransformId;
use indextree::{Arena, NodeId};
use itertools::Itertools;
use nalgebra::Isometry3;
use std::collections::{HashMap, HashSet};

/// Implements a single transform graph at a single point in time (without parallel channels).
///
pub struct IsometryGraph {
    arena: Arena<Isometry3<f64>>,
    transform_id_to_node_id_map: HashMap<TransformId, NodeId>,
    node_id_to_transform_id_map: HashMap<NodeId, TransformId>,
    // frame_id_to_node_id_map: HashMap<FrameId, NodeId>,
    // child_frame_id_to_node_id_map: HashMap<FrameId, NodeId>,
}

impl IsometryGraph {
    pub fn new(isometry_transforms: HashMap<TransformId, Isometry3<f64>>) -> Self {
        assert!(
            !isometry_transforms.is_empty(),
            "Must contain at least one transform."
        );
        // TODO: add checks again
        /*
        assert!(
            transforms
                .values()
                .flat_map(|t| t)
                .all(|t| t.timestamp == transforms.values().first().unwrap().timestamp),
            "Must all contain the same timestamp."
        );*/

        let mut arena: Arena<Isometry3<f64>> = Arena::new();

        let mut transform_id_to_node_id_map: HashMap<TransformId, NodeId> = HashMap::new();
        let mut node_id_to_transform_id_map: HashMap<NodeId, TransformId> = HashMap::new();
        let mut frame_id_to_node_id_map: HashMap<FrameId, NodeId> = HashMap::new();
        let mut child_frame_id_to_node_id_map: HashMap<FrameId, NodeId> = HashMap::new();

        for (transform_id, current_isometry) in isometry_transforms {
            let current_node_id = arena.new_node(current_isometry);
            // let current_child_node_id =

            transform_id_to_node_id_map.insert(transform_id.clone(), current_node_id);
            node_id_to_transform_id_map.insert(current_node_id, transform_id.clone());

            frame_id_to_node_id_map.insert(transform_id.frame_id.clone(), current_node_id);
            child_frame_id_to_node_id_map
                .insert(transform_id.child_frame_id.clone(), current_node_id);
        }

        for (current_transform_id, current_node_id) in &transform_id_to_node_id_map {
            // println!("check for appending: current_transform_id: {}, current_node_id: {}", current_transform_id, current_node_id);

            if let Some((_, &current_parent_node_id)) = transform_id_to_node_id_map
                .iter()
                .find(|t| t.0.child_frame_id == current_transform_id.frame_id)
            {
                current_parent_node_id
                    .checked_append(*current_node_id, &mut arena)
                    .unwrap();
            }

            /*if let Some(&current_child_node_id) =
                frame_id_to_node_id_map.get(&current_transform_id.child_frame_id)
            {
                println!("append: current_node_id: {}, current_child_node_id: {}", current_node_id, current_child_node_id);
                current_node_id.checked_append(current_child_node_id, &mut arena).unwrap();
            }*/
        }

        println!("{:?}", arena);
        // debugging
        for current_node in arena.iter() {
            let current_node_id = arena.get_node_id(current_node).unwrap();
            let current_transform_id = node_id_to_transform_id_map.get(&current_node_id).unwrap();
            println!(
                "current node (node_id={}): frame_id={} child_frame_id={}",
                current_node_id, current_transform_id.frame_id, current_transform_id.child_frame_id
            );

            for current_ancestor_node_id in current_node_id.ancestors(&arena) {
                let current_ancestor_transform_id = node_id_to_transform_id_map
                    .get(&current_ancestor_node_id)
                    .unwrap();
                println!(
                    " - (node_id={}): frame_id={} child_frame_id={}",
                    current_ancestor_node_id,
                    current_ancestor_transform_id.frame_id,
                    current_ancestor_transform_id.child_frame_id
                );
            }
            println!();
        }

        Self {
            arena,
            transform_id_to_node_id_map,
            node_id_to_transform_id_map,
            //frame_id_to_node_id_map,
            //child_frame_id_to_node_id_map,
        }
    }

    pub fn get_frame_ids(&self) -> HashSet<FrameId> {
        self.transform_id_to_node_id_map
            .keys()
            .fold(Vec::<FrameId>::new(), |mut acc, x| {
                acc.push(x.frame_id.clone());
                acc.push(x.child_frame_id.clone());
                acc
            })
            .into_iter()
            .collect()
    }

    pub fn get_isometry(&self, transform_id: &TransformId) -> Isometry3<f64> {
        assert!(
            self.transform_id_to_node_id_map
                .keys()
                .any(|k| k.frame_id == transform_id.frame_id),
            "Transform graph must contain a transform with frame_id: {}",
            &transform_id.frame_id
        );
        assert!(
            self.transform_id_to_node_id_map
                .keys()
                .any(|k| k.child_frame_id == transform_id.child_frame_id),
            "Transform graph must contain a transform with child_frame_id: {}",
            &transform_id.child_frame_id
        );

        let child_node_id = self
            //.child_frame_id_to_node_id_map
            .transform_id_to_node_id_map
            .iter()
            .find(|(t, _)| t.child_frame_id == transform_id.child_frame_id)
            //.get(&transform_id.child_frame_id)
            .map(|r| r.1)
            .unwrap();

        let mut isometry: Isometry3<f64> = Isometry3::identity();
        let ancestors = child_node_id.ancestors(&self.arena);
        let anc_pr = ancestors.clone().map(|a| a.to_string()).join(" ");
        println!("ancestor ids: {}", anc_pr);
        //let mut selected_nodes: Vec<&Transform> = vec![];
        for current_ancestor_node_id in ancestors {
            let current_node = self.arena.get(current_ancestor_node_id).unwrap().get();
            let current_transform_id = self
                .node_id_to_transform_id_map
                .get(&current_ancestor_node_id)
                .unwrap();

            println!(
                "current node: frame_id={} child_frame_id={}",
                &current_transform_id.frame_id, &current_transform_id.child_frame_id
            );
            isometry = current_node * isometry;
            println!("isometry after: {}", isometry);

            if current_transform_id.frame_id == transform_id.frame_id {
                println!("doing the break");
                break;
            }
        }

        dbg!("isometry final: {}", isometry);
        isometry
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

        let isometry_graph = IsometryGraph::new(isometry_transforms);
        let result = isometry_graph.get_isometry(&TransformId::new(
            FrameId::from("slam_map"),
            FrameId::from("lidar_front_right"),
        ));

        assert_eq!(result.translation, Translation3::new(50.0, 0.0, 0.0));
    }
}
