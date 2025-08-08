use crate::octree::OctantIndex;
use petgraph::Graph;
use petgraph::graph::NodeIndex;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct OctreeOccupancyGraph {
    graph: Graph<OctantIndex, ()>,
    octant_index_to_node_index_map: HashMap<OctantIndex, NodeIndex>,
}

impl Default for OctreeOccupancyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl OctreeOccupancyGraph {
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            octant_index_to_node_index_map: HashMap::new(),
        }
    }

    pub fn get_occupied_cell_indices_of_level(&self, level: u32) -> Vec<OctantIndex> {
        self.octant_index_to_node_index_map
            .keys()
            .filter(|&i| i.level == level)
            .copied()
            .collect()
    }

    /// Returns true if the cell with index or at least one of its child cell is occupied with
    /// content.
    pub fn is_cell_occupied(&self, index: OctantIndex) -> bool {
        self.octant_index_to_node_index_map.contains_key(&index)
    }

    /// Adds the occupancy of the [octant_index] cell and its parents.
    pub fn add_cell_occupancy(&mut self, octant_index: OctantIndex) {
        let mut current_octant_index = octant_index;

        while let Some(current_parent_octant_index) = current_octant_index.get_parent() {
            let current_node_index = self.get_or_insert_occupancy_tree_node(current_octant_index);
            let current_parent_node_index =
                self.get_or_insert_occupancy_tree_node(current_parent_octant_index);

            self.graph
                .add_edge(current_parent_node_index, current_node_index, ());

            current_octant_index = current_parent_octant_index;
        }
    }

    fn get_or_insert_occupancy_tree_node(&mut self, octant_index: OctantIndex) -> NodeIndex {
        if let Some(node_index) = self.octant_index_to_node_index_map.get(&octant_index) {
            return *node_index;
        }

        let new_node = self.graph.add_node(octant_index);
        self.octant_index_to_node_index_map
            .insert(octant_index, new_node);
        new_node
    }
}
