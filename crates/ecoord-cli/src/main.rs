mod cli;
mod commands;
mod error;
mod util;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use clap::Parser;
use nalgebra::Vector3;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Stats { ecoord_file_path } => {
            commands::stats::run(ecoord_file_path)?;
        }
        Commands::ConvertFromKittiFormat {
            kitti_file_path,
            ecoord_file_path,
            pretty,
            start_date_time,
            end_date_time,
            trajectory_parent_frame_id,
            trajectory_child_frame_id,
            global_frame_id,
            local_origin_offset,
        } => {
            let local_origin_offset: Option<Vector3<f64>> = match local_origin_offset.len() {
                3 => Some(Vector3::new(
                    local_origin_offset[0],
                    local_origin_offset[1],
                    local_origin_offset[2],
                )),
                0 => None,
                _ => {
                    panic!("local_origin_offset must be of length 3");
                }
            };

            commands::convert_kitti::run(
                kitti_file_path,
                ecoord_file_path,
                *start_date_time,
                *end_date_time,
                trajectory_parent_frame_id.clone(),
                trajectory_child_frame_id.clone(),
                global_frame_id.clone(),
                local_origin_offset,
                *pretty,
            )?;
        }
    };

    Ok(())
}
