use crate::coords::error::Error;
use crate::coords::error::Error::{InvalidNumber, LowerBoundExceedsUpperBound};
use nalgebra::{Point3, Vector3};
use std::fmt::Debug;

pub trait HasAabb {
    fn center(&self) -> Point3<f64>;

    fn min(&self) -> Point3<f64>;

    fn max(&self) -> Point3<f64>;
}

impl HasAabb for Point3<f64> {
    fn center(&self) -> Point3<f64> {
        *self
    }

    fn min(&self) -> Point3<f64> {
        *self
    }

    fn max(&self) -> Point3<f64> {
        *self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisAlignedBoundingBox {
    lower_bound: Point3<f64>,
    upper_bound: Point3<f64>,
}

impl AxisAlignedBoundingBox {
    pub fn new(lower_bound: Point3<f64>, upper_bound: Point3<f64>) -> Result<Self, Error> {
        if (lower_bound.x > upper_bound.x)
            || (lower_bound.y > upper_bound.y)
            || (lower_bound.z > upper_bound.z)
        {
            return Err(LowerBoundExceedsUpperBound);
        }

        Ok(Self {
            lower_bound,
            upper_bound,
        })
    }

    pub fn lower_bound(&self) -> Point3<f64> {
        self.lower_bound
    }

    pub fn upper_bound(&self) -> Point3<f64> {
        self.upper_bound
    }

    pub fn diagonal(&self) -> Vector3<f64> {
        self.upper_bound - self.lower_bound
    }

    pub fn volume(&self) -> f64 {
        let diagonal = self.diagonal();
        diagonal.x * diagonal.y * diagonal.z
    }

    pub fn get_center(&self) -> Point3<f64> {
        let diagonal = self.upper_bound - self.lower_bound;
        self.lower_bound + diagonal / 2.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AxisAlignedBoundingCube {
    lower_bound: Point3<f64>,
    edge_length: f64,

    // internally determined upper bound
    upper_bound: Point3<f64>,
}

impl AxisAlignedBoundingCube {
    pub fn new(lower_bound: Point3<f64>, edge_length: f64) -> Result<Self, Error> {
        if edge_length <= 0.0 {
            return Err(InvalidNumber);
        }

        Ok(Self::new_unchecked(lower_bound, edge_length))
    }

    pub fn new_unchecked(lower_bound: Point3<f64>, edge_length: f64) -> Self {
        let upper_bound = lower_bound + Vector3::new(edge_length, edge_length, edge_length);
        Self {
            lower_bound,
            edge_length,
            upper_bound,
        }
    }

    /// Builds a cube that encloses the AABB with a power-of-two edge length.
    pub fn from_power_of_two_enclosing_box(bounding_box: &AxisAlignedBoundingBox) -> Self {
        let center = bounding_box.get_center();
        let diagonal = bounding_box.diagonal();
        let max_extent = diagonal.x.max(diagonal.y).max(diagonal.z);

        let edge_length = next_strict_power_of_two_f64(max_extent);
        let half_edge_length = edge_length / 2.0;
        let lower_bound =
            center - Vector3::new(half_edge_length, half_edge_length, half_edge_length);

        Self::new_unchecked(lower_bound, edge_length)
    }

    pub fn center(&self) -> Point3<f64> {
        let half_edge_length = self.half_edge_length();
        self.lower_bound + Vector3::new(half_edge_length, half_edge_length, half_edge_length)
    }

    pub fn edge_length(&self) -> f64 {
        self.edge_length
    }
    pub fn half_edge_length(&self) -> f64 {
        self.edge_length / 2.0
    }

    pub fn get_lower_bound(&self) -> Point3<f64> {
        self.lower_bound
    }

    pub fn get_upper_bound(&self) -> Point3<f64> {
        self.upper_bound
    }

    pub fn diagonal(&self) -> Vector3<f64> {
        Vector3::new(self.edge_length, self.edge_length, self.edge_length)
    }

    pub fn volume(&self) -> f64 {
        self.edge_length() * self.edge_length() * self.edge_length()
    }

    /// Checks if the point lies within the AABB using **half-open** bounds: [min, max)
    pub fn contains_point(&self, point: &Point3<f64>) -> bool {
        if point.x < self.lower_bound.x {
            return false;
        }
        if point.x >= self.upper_bound.x {
            return false;
        }
        if point.y < self.lower_bound.y {
            return false;
        }
        if point.y >= self.upper_bound.y {
            return false;
        }
        if point.z < self.lower_bound.z {
            return false;
        }
        if point.z >= self.upper_bound.z {
            return false;
        }

        true
    }

    /// Checks if the point lies within the AABB using **closed** bounds: [min, max]
    pub fn contains_point_closed(&self, point: &Point3<f64>) -> bool {
        if point.x < self.lower_bound.x {
            return false;
        }
        if point.x > self.upper_bound.x {
            return false;
        }
        if point.y < self.lower_bound.y {
            return false;
        }
        if point.y > self.upper_bound.y {
            return false;
        }
        if point.z < self.lower_bound.z {
            return false;
        }
        if point.z > self.upper_bound.z {
            return false;
        }

        true
    }

    pub fn get_sub_cube(&self, x_half: bool, y_half: bool, z_half: bool) -> Self {
        let cube_half_edge_length = self.half_edge_length();

        let sub_cube_lower_bound_x = if x_half {
            self.lower_bound.x + cube_half_edge_length
        } else {
            self.lower_bound.x
        };
        let sub_cube_lower_bound_y = if y_half {
            self.lower_bound.y + cube_half_edge_length
        } else {
            self.lower_bound.y
        };
        let sub_cube_lower_bound_z = if z_half {
            self.lower_bound.z + cube_half_edge_length
        } else {
            self.lower_bound.z
        };

        let sub_cube_lower_bound = Point3::new(
            sub_cube_lower_bound_x,
            sub_cube_lower_bound_y,
            sub_cube_lower_bound_z,
        );
        Self::new(sub_cube_lower_bound, self.half_edge_length()).expect("should work")
    }
}

fn next_strict_power_of_two_f64(x: f64) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }

    let exponent = x.log2().floor() + 1.0;
    2f64.powf(exponent)
}
