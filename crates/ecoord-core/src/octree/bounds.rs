use crate::octree::OctantIndex;
use crate::{AxisAlignedBoundingBox, AxisAlignedBoundingCube};
use nalgebra::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OctreeBounds {
    /// Bounding box of the content items
    bounding_box: AxisAlignedBoundingBox,

    /// Enclosing cube enlarged to have a power of two edge lengths
    enclosing_cube: AxisAlignedBoundingCube,
}

impl OctreeBounds {
    pub fn new(bounding_box: AxisAlignedBoundingBox) -> Self {
        let enclosing_cube =
            AxisAlignedBoundingCube::from_power_of_two_enclosing_box(&bounding_box);

        Self {
            bounding_box,
            enclosing_cube,
        }
    }

    pub fn bounding_box(&self) -> &AxisAlignedBoundingBox {
        &self.bounding_box
    }

    /// Enclosing cube enlarged to have a power of two edge lengths
    pub fn enclosing_cube(&self) -> &AxisAlignedBoundingCube {
        &self.enclosing_cube
    }

    /// Calculates the axis-aligned bounding cube for a specific octant within the enclosing cube.
    pub fn get_octant_bounding_cube(&self, index: OctantIndex) -> AxisAlignedBoundingCube {
        let octant_edge_length =
            self.enclosing_cube.edge_length() / (2usize.pow(index.level) as f64);

        let lower_bound = self.enclosing_cube.get_lower_bound();
        let octant_lower_bound_x = lower_bound.x + octant_edge_length * (index.x as f64);
        let octant_lower_bound_y = lower_bound.y + octant_edge_length * (index.y as f64);
        let octant_lower_bound_z = lower_bound.z + octant_edge_length * (index.z as f64);
        let octant_lower_bound = Point3::new(
            octant_lower_bound_x,
            octant_lower_bound_y,
            octant_lower_bound_z,
        );

        AxisAlignedBoundingCube::new(octant_lower_bound, octant_edge_length).expect("should work")
    }
}
