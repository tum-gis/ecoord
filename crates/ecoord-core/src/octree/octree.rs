use crate::AxisAlignedBoundingBox;
use crate::coords::bounding_box::HasAabb;
use crate::coords::error::Error;
use crate::octree::{OctantIndex, OctreeBounds, OctreeOccupancyGraph};
use nalgebra::Point3;
use rand::SeedableRng;
use rand::prelude::{SliceRandom, StdRng};
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct Octree<T: HasAabb + Sync + Send + Clone + 'static> {
    bounds: OctreeBounds,
    occupancy_graph: OctreeOccupancyGraph,
    cells: HashMap<OctantIndex, Vec<T>>,
}

impl<T: HasAabb + Sync + Send + Clone + 'static + Debug> Octree<T> {
    pub fn new(
        items: Vec<T>,
        max_items_per_octant: usize,
        shuffle_seed_number: Option<u64>,
    ) -> Result<Self, crate::Error> {
        let (bounds, occupancy_graph, items_per_octant) =
            compute_octree(items, max_items_per_octant, shuffle_seed_number)?;

        Ok(Self {
            bounds,
            occupancy_graph,
            cells: items_per_octant,
        })
    }

    pub fn from_raw_parts(
        bounds: OctreeBounds,
        occupancy_graph: OctreeOccupancyGraph,
        cells: HashMap<OctantIndex, Vec<T>>,
    ) -> Result<Self, crate::Error> {
        Ok(Self {
            bounds,
            occupancy_graph,
            cells,
        })
    }

    /// Returns the bounds of the octree.
    pub fn bounds(&self) -> &OctreeBounds {
        &self.bounds
    }

    /// Returns the occupancy graph of the octree.
    pub fn occupancy_graph(&self) -> &OctreeOccupancyGraph {
        &self.occupancy_graph
    }

    /// Returns the cells of the octree.
    pub fn cells(&self) -> &HashMap<OctantIndex, Vec<T>> {
        &self.cells
    }

    /// Returns the set of octant indices that contain data.
    pub fn cell_indices(&self) -> HashSet<OctantIndex> {
        self.cells.keys().copied().collect()
    }

    /// Returns the number of octants that contain data.
    pub fn cell_count(&self) -> usize {
        self.cells.len()
    }

    /// Returns the data associated with the given octant index, if any.
    pub fn cell(&self, index: OctantIndex) -> Option<&Vec<T>> {
        self.cells.get(&index)
    }

    pub fn get_max_occupied_level(&self) -> Option<u32> {
        self.cells.keys().map(|x| x.level).max()
    }

    pub fn contains_content_cells(&self, index: OctantIndex) -> bool {
        self.cells.contains_key(&index)
    }
}

struct IntermediateResult<'a, T: HasAabb> {
    octant_index: OctantIndex,
    assigned_items: Option<Vec<&'a T>>,
    remaining_items: Option<Vec<&'a T>>,
}

fn compute_octree<T: HasAabb + Sync + Send + Clone + 'static + Debug>(
    mut items: Vec<T>,
    max_items_per_octant: usize,
    shuffle_seed_number: Option<u64>,
) -> Result<
    (
        OctreeBounds,
        OctreeOccupancyGraph,
        HashMap<OctantIndex, Vec<T>>,
    ),
    Error,
> {
    let octree_bounds = derive_octree_bounds(&items)?;
    let mut occupancy_graph = OctreeOccupancyGraph::new();

    shuffle_items_if_needed(&mut items, shuffle_seed_number);

    let mut pending_items = initialize_pending_items(&items);
    let mut final_items: HashMap<OctantIndex, Vec<&T>> = HashMap::new();

    while !pending_items.is_empty() {
        let results = sub_divide(pending_items, &octree_bounds, max_items_per_octant);

        results
            .iter()
            .for_each(|r| occupancy_graph.add_cell_occupancy(r.octant_index));

        final_items.extend(
            results
                .iter()
                .filter(|r| r.assigned_items.is_some())
                .map(|r| (r.octant_index, r.assigned_items.clone().unwrap()))
                .collect::<Vec<(OctantIndex, Vec<&T>)>>(),
        );

        pending_items = results
            .into_iter()
            .filter(|r| r.remaining_items.is_some())
            .map(|r| (Some(r.octant_index), r.remaining_items.unwrap()))
            .collect();
    }

    Ok((
        octree_bounds,
        occupancy_graph,
        final_items
            .into_iter()
            .map(|(k, v)| (k, v.into_iter().cloned().collect()))
            .collect(),
    ))
}

fn derive_octree_bounds<T: HasAabb + Clone>(items: &[T]) -> Result<OctreeBounds, Error> {
    let min_x = items
        .iter()
        .map(|i| i.min().x)
        .reduce(f64::min)
        .ok_or(Error::NoMinValue)?;
    let min_y = items
        .iter()
        .map(|i| i.min().y)
        .reduce(f64::min)
        .ok_or(Error::NoMinValue)?;
    let min_z = items
        .iter()
        .map(|i| i.min().z)
        .reduce(f64::min)
        .ok_or(Error::NoMinValue)?;
    let max_x = items
        .iter()
        .map(|i| i.max().x)
        .reduce(f64::max)
        .ok_or(Error::NoMaxValue)?;
    let max_y = items
        .iter()
        .map(|i| i.max().y)
        .reduce(f64::max)
        .ok_or(Error::NoMaxValue)?;
    let max_z = items
        .iter()
        .map(|i| i.max().z)
        .reduce(f64::max)
        .ok_or(Error::NoMaxValue)?;

    let bounding_box = AxisAlignedBoundingBox::new(
        Point3::new(min_x, min_y, min_z),
        Point3::new(max_x, max_y, max_z),
    )?;
    let octree_bounds = OctreeBounds::new(bounding_box);

    Ok(octree_bounds)
}

fn shuffle_items_if_needed<T>(items: &mut Vec<T>, shuffle_seed_number: Option<u64>) {
    if let Some(seed_number) = shuffle_seed_number {
        let mut rng = StdRng::seed_from_u64(seed_number);
        items.shuffle(&mut rng);
    }
}

fn initialize_pending_items<T>(items: &[T]) -> HashMap<Option<OctantIndex>, Vec<&T>> {
    let mut pending_items = HashMap::new();
    pending_items.insert(None, items.iter().collect());
    pending_items
}

fn sub_divide<'a, T: HasAabb + Sync + Send>(
    pending_items: HashMap<Option<OctantIndex>, Vec<&'a T>>,
    octree_bounds: &'a OctreeBounds,
    max_items_per_octant: usize,
) -> Vec<IntermediateResult<'a, T>> {
    let current_octant_indices: Vec<(Option<OctantIndex>, OctantIndex)> =
        get_child_pairs(&pending_items.keys().copied().collect());

    let results: Vec<IntermediateResult<T>> = current_octant_indices
        .par_iter()
        .map(
            |(current_parent_octant_index, current_child_octant_index)| {
                let current_bounding_cube =
                    octree_bounds.get_octant_bounding_cube(*current_child_octant_index);
                let pending_items_for_octant = pending_items
                    .get(current_parent_octant_index)
                    .expect("should exist");

                let mut items_within_cube: Vec<&T> = pending_items_for_octant
                    .iter()
                    .filter(|x| current_bounding_cube.contains_point(&x.center()))
                    .copied()
                    .collect();
                let remaining_items: Option<Vec<&T>> =
                    if items_within_cube.len() > max_items_per_octant {
                        Some(items_within_cube.split_off(max_items_per_octant))
                    } else {
                        None
                    };
                let assigned_items = if items_within_cube.is_empty() {
                    None
                } else {
                    Some(items_within_cube)
                };

                IntermediateResult {
                    octant_index: *current_child_octant_index,
                    assigned_items,
                    remaining_items,
                }
            },
        )
        .collect();

    results
}

fn get_child_pairs(
    octant_indices: &HashSet<Option<OctantIndex>>,
) -> Vec<(Option<OctantIndex>, OctantIndex)> {
    octant_indices
        .iter()
        .flat_map(|parent| generate_child_pairs(*parent))
        .collect()
}

fn generate_child_pairs(
    parent: Option<OctantIndex>,
) -> impl Iterator<Item = (Option<OctantIndex>, OctantIndex)> {
    match parent {
        Some(parent_index) => {
            let children = parent_index.get_children();
            itertools::Either::Left(
                children
                    .into_iter()
                    .map(move |child| (Some(parent_index), child)),
            )
        }
        None => itertools::Either::Right(std::iter::once((None, OctantIndex::origin()))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use nalgebra::Point3;

    #[test]
    fn test_octant_enclosing_cube_boundary_issue() {
        let point_a = Point3::new(691140.231908248, 5338107.586181451, 483.81417527816086);
        let point_b = Point3::new(691201.311408248, 5338168.665681452, 544.8936752782698);

        let bounds = derive_octree_bounds(&[point_a, point_b]).expect("should work");

        assert!(bounds.enclosing_cube().contains_point(&point_a));
        assert!(bounds.enclosing_cube().contains_point(&point_b));
    }

    #[test]
    fn test_octant_enclosing_cube_boundary_issue_2() {
        let point_a = Point3::new(0.0, 0.0, 0.0);
        let point_b = Point3::new(64.0f64, 64.0f64, 64.0f64);
        let point_b_next_up = Point3::new(64.0f64.next_up(), 64.0f64.next_up(), 64.0f64.next_up());

        let bounds = derive_octree_bounds(&[point_a, point_b]).expect("should work");

        assert!(bounds.enclosing_cube().contains_point(&point_a));
        assert!(bounds.enclosing_cube().contains_point(&point_b));
    }
}
