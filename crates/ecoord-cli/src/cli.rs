use crate::util::parse_timestamp;
use chrono::{DateTime, Utc};
use clap::ValueHint;
use clap::{Parser, Subcommand};
use ecoord::FrameId;
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Derive statistics from an ecoord document
    Stats {
        /// Path to the ecoord document
        #[clap(short, long)]
        ecoord_file_path: PathBuf,
    },

    /// Convert from KITTI
    ConvertFromKittiFormat {
        /// Path to the KITTI document
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        kitti_file_path: PathBuf,

        /// Path to the ecoord document
        #[clap(short, long, value_hint = ValueHint::FilePath)]
        ecoord_file_path: PathBuf,

        /// The start time of the import in UTC.
        /// Example: 2020-04-12 22:10:57.123456789 +00:00
        /// If not provided, the import starts from the beginning
        #[clap(long, value_parser = parse_timestamp)]
        start_date_time: DateTime<Utc>,

        /// The start time of the import in UTC.
        /// Example: 2020-04-12 22:10:57.123456789 +00:00
        /// If not provided, the import starts from the beginning
        #[clap(long, value_parser = parse_timestamp)]
        end_date_time: DateTime<Utc>,

        #[clap(long, default_value_t = FrameId::local())]
        trajectory_parent_frame_id: FrameId,

        #[clap(long, default_value_t = FrameId::base_link())]
        trajectory_child_frame_id: FrameId,

        #[clap(long, default_value_t = FrameId::global())]
        global_frame_id: FrameId,

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        local_origin_offset: Vec<f64>,

        /// Format the output with indentation and line breaks for readability
        #[clap(short, long, default_value_t = false)]
        pretty: bool,
    },
}
