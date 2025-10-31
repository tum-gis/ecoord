use crate::util::parse_timestamp;
use chrono::{DateTime, Utc};
use clap::ValueHint;
use clap::{Parser, Subcommand};
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

        #[clap(long, number_of_values = 3, allow_hyphen_values = true)]
        world_offset: Vec<f64>,

        /// Format the output with indentation and line breaks for readability
        #[clap(short, long, default_value_t = false)]
        pretty: bool,
    },

    /// Convert from tabular format, such as CSV
    ConvertFromTabularFormat {
        /// Path to the input file or directory containing tabular files
        #[arg(short, long, value_name = "PATH", value_hint = ValueHint::AnyPath)]
        input_path: PathBuf,

        /// Path to the output ecoord file or directory
        #[clap(short, long, value_hint = ValueHint::AnyPath)]
        output_path: PathBuf,

        /// Channel ID for the trajectory data (e.g., "slam" or "rtk_gnss")
        #[clap(long)]
        trajectory_channel_id: String,

        /// Frame ID representing the reference frame for the trajectory (e.g., "world")
        #[clap(long)]
        trajectory_frame_id: String,

        /// Frame ID of the moving body/child frame in the trajectory transform (e.g., "left_lidar_sensor")
        #[clap(long)]
        trajectory_child_frame_id: String,

        /// Format the output with indentation and line breaks for readability
        #[clap(short, long, default_value_t = false)]
        pretty: bool,
    },
}
