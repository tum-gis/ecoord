use crate::util::parse_timestamp;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};

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
        ecoord_file_path: String,
    },

    /// Convert from KITTI
    ConvertFromKittiFormat {
        /// Path to the KITTI document
        #[clap(short, long)]
        kitti_file_path: String,

        /// Path to the ecoord document
        #[clap(short, long)]
        ecoord_file_path: String,

        /// The start time of the import in UTC.
        /// Example: 2020-04-12 22:10:57.123456789 +00:00
        /// If not provided, the import starts from the beginning
        #[clap(long, value_parser = parse_timestamp)]
        start_date_time: DateTime<Utc>,

        /// The start time of the import in UTC.
        /// Example: 2020-04-12 22:10:57.123456789 +00:00
        /// If not provided, the import starts from the beginning
        #[clap(long, value_parser = parse_timestamp)]
        stop_date_time: DateTime<Utc>,

        #[clap(long, default_value_t = String::from("slam"))]
        trajectory_channel_id: String,

        #[clap(long, default_value_t = String::from("world_offset"))]
        trajectory_frame_id: String,

        #[clap(long, default_value_t = String::from("base_link"))]
        trajectory_child_frame_id: String,

        #[clap(long, default_value_t = String::from("world_offset"))]
        world_offset_channel_id: String,

        #[clap(long, default_value_t = String::from("world"))]
        world_frame_id: String,

        /// Offset point cloud
        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        world_offset: Vec<f64>,
    },
}
