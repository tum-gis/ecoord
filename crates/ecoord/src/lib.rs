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
//! The transforms from a frame to another are assigned to channels, which enables the activation
//! and deactivation of selected channels. If multiple channels describe the same transform from a
//! frame to another frame, the channel with the highest prioritization number is used.
//! If multiple channels have the same prioritization number, the alphabetically sorted last
//! channel name is used.
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
//!     - `transforms`
//!         - `channel_id`: [String]
//!         - `frame_id`: [String]
//!         - `child_frame_id`: [String]
//!         - `timestamp`:
//!             - sec: [i32]
//!             - nanosec: [u32]
//!         - `duration`: [Option] durations must lead to overlaps (no undefined times)
//!             - sec: [i32]
//!             - nanosec: [u32]
//!         - `translation`
//!             - `x`: [f64]
//!             - `y`: [f64]
//!             - `z`: [f64]
//!         - `rotation`: in Quaternion
//!             - `x`: [f64]
//!             - `y`: [f64]
//!             - `z`: [f64]
//!             - `w`: [f64]
//!     - `channel_info`: additional information on channels
//!         - `ìd`: [String]
//!         - `priority`: [Option]<[i32]>
//!             - default: `0`
//!             - if multiple channels hold transforms for the same frame to child frame, the one with the higher priority is selected
//!     - `frame_info`: additional information on frames
//!         - `id`: [String]
//!             - unique identifier
//!         - `crs_epsg`: [Option]<[i32]>
//!     - `transform_info`
//!         - `frame_id`: [String]
//!         - `child_frame_id`: [String]
//!         - `interpolation_method`: [Option]<[String]>
//!             - `step` (default): piecewise constant interpolation
//!             - `linear`: linear interpolation
//!

pub use ecoord_core::{
    merge, ChannelId, ChannelInfo, Error, FrameId, FrameInfo, InterpolationMethod, ReferenceFrames,
    SphericalPoint3, Transform, TransformId, TransformInfo, UnitSphericalPoint3,
};

pub use ecoord_io as io;
pub use ecoord_transform as transform;
