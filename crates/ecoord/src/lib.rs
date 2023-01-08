//! Library for representing 3D coordinate frames and transforms between them using the file
//! extension `.ecoo`.
//!
//! The position of an object in 3D space can be described geometrically using an
//! [inertial frame of reference](https://en.wikipedia.org/wiki/Inertial_frame_of_reference).
//! If the transform (translation and rotation) between two reference frames is known, the object's
//! position can be determined for each frame. As some objects, like pedestrians or UAVs,
//! move in space, a reference frame can be time-dependent.
//!
//!
//! TODO explain:
//! - timestamps and interpolations
//! - channels with priorities
//!
//! If multiple channels describe the same transform from a frame to another frame,
//! the channel with the highest prioritization number is used. If multiple channels have
//! the same prioritization number, the alphabetically sorted last channel name is used.
//!
//!
//!
//! # Overview
//!
//! # Data Structure
//!
//! For de/serializing JSON is used in three forms:
//! - `frames.json`: readable JSON
//! - `name.ecoo`: compressed JSON using the [ZStandard](http://facebook.github.io/zstd/) compression algorithm
//!
//! Document Structure:
//! - document
//!     - `transforms`
//!         - `channel_id`: [String](String)
//!         - `frame_id`: [String](String)
//!         - `child_frame_id`: [String](String)
//!         - `timestamp`:
//!             - sec: [i32](i32)
//!             - nanosec: [u32](u32)
//!         - `duration`: [Option](Option) durations must lead to overlaps (no undefined times)
//!             - sec: [i32](i32)
//!             - nanosec: [u32](u32)
//!         - `translation`
//!             - `x`: [f64](f64)
//!             - `y`: [f64](f64)
//!             - `z`: [f64](f64)
//!         - `rotation`: in Quaternion
//!             - `x`: [f64](f64)
//!             - `y`: [f64](f64)
//!             - `z`: [f64](f64)
//!             - `w`: [f64](f64)
//!     - `channel_info`: additional information on channels
//!         - `ìd` [String](String)
//!         - `priority`: [Option](Option)<[i32](i32)>
//!             - default: `0`
//!             - if multiple channels hold transforms for the same frame to child frame, the one with the higher priority is selected
//!     - `frame_info`: additional information on frames
//!         - `id`: [String](String)
//!             - unique identifier
//!         - `crs_epsg`: [Option](Option)<[i32](i32)>
//!     - `transform_info`
//!         - `frame_id`: [String](String)
//!         - `child_frame_id`: [String](String)
//!         - `interpolation_method`: [Option](Option)<[String](String)>
//!             - `none` (default)
//!             - `linear`
//!             - `cubic`
//!

pub use ecoord_core::{
    merge, ChannelId, ChannelInfo, FrameId, FrameInfo, InterpolationMethod, ReferenceFrames,
    Transform, TransformId, TransformInfo,
};

pub use ecoord_io as io;
