//! A library for transforming between 3D coordinate frames.
//!
//! The position of an object in 3D space can be described geometrically using an
//! [inertial frame of reference](https://en.wikipedia.org/wiki/Inertial_frame_of_reference).
//! If the transform (translation and rotation) between two reference frames is known, the object's
//! position can be determined for each frame.
//!
//! As some objects, like pedestrians or UAVs, move in space, a reference frame can be
//! time-dependent. Each transform object has a timestamp defined in seconds and nanoseconds,
//! whereby different interpolation strategies, such as step-wise or linear, can be applied.
//!
//!
//!
//! # Overview
//!
//! # Data Structure
//!
//! For de/serializing JSON is used in three forms:
//! - `ecoord.json`: readable JSON
//! - `file_name.ecoord.json` prefixing with a file name
//! - `file_name.ecoord.json.zst`: compressed JSON using the [ZStandard](http://facebook.github.io/zstd/) compression algorithm
//!
//! Document Structure:
//! - document
//!     - `edges`
//!         - `parent_frame_id`: [String]
//!         - `child_frame_id`: [String]
//!         - `interpolation_method`: [Option]<[String]>
//!             - `step` (default): piecewise constant interpolation
//!             - `linear`: linear interpolation
//!         - `extrapolation_method`: [Option]<[String]>
//!             - `constant` (default): constant extrapolation
//!             - `linear`: linear extrapolation
//!         - `samples`:
//!             - `timestamp`:
//!                 - sec: [i32]
//!                 - nanosec: [u32]
//!             - `translation`
//!                 - `x`: [f64]
//!                 - `y`: [f64]
//!                 - `z`: [f64]
//!             - `rotation`: in Quaternion
//!                 - `x`: [f64]
//!                 - `y`: [f64]
//!                 - `z`: [f64]
//!                 - `w`: [f64]
//!     - `frame_info`: additional information on frames
//!         - `id`: [String]
//!             - unique identifier
//!         - `description`: [Option]<[String]>
//!         - `crs_epsg`: [Option]<[i32]>

pub use ecoord_core::{
    AxisAlignedBoundingBox, AxisAlignedBoundingCube, DynamicTransform, Error, ExtrapolationMethod,
    FrameId, FrameInfo, HasAabb, InterpolationMethod, SphericalPoint3, StaticTransform,
    TimedTransform, Transform, TransformEdge, TransformId, TransformTree, UnitSphericalPoint3,
    merge, octree,
};

pub use ecoord_io as io;
pub use ecoord_transform as transform;
