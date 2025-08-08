mod cli;
mod commands;
mod error;
mod util;

use anyhow::Result;

use crate::cli::{Cli, Commands};
use clap::Parser;
use ecoord::{ChannelId, FrameId};
use nalgebra::Vector3;
use std::path::PathBuf;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Stats { ecoord_file_path } => {
            let ecoord_file_path = PathBuf::from(ecoord_file_path);

            commands::stats::run(ecoord_file_path)?;
        }
        Commands::ConvertFromKittiFormat {
            kitti_file_path,
            ecoord_file_path,
            start_date_time,
            stop_date_time,
            trajectory_channel_id,
            trajectory_frame_id,
            trajectory_child_frame_id,
            world_offset_channel_id,
            world_frame_id,
            world_offset,
        } => {
            let kitti_file_path = PathBuf::from(kitti_file_path);
            let ecoord_file_path = PathBuf::from(ecoord_file_path);
            let trajectory_channel_id = ChannelId::from(trajectory_channel_id.clone());
            let trajectory_frame_id = FrameId::from(trajectory_frame_id.clone());
            let trajectory_child_frame_id = FrameId::from(trajectory_child_frame_id.clone());
            let world_offset_channel_id = ChannelId::from(world_offset_channel_id.clone());
            let world_frame_id = FrameId::from(world_frame_id.clone());
            let world_offset: Option<Vector3<f64>> = match world_offset.len() {
                3 => Some(Vector3::new(
                    world_offset[0],
                    world_offset[1],
                    world_offset[2],
                )),
                0 => None,
                _ => {
                    panic!("world_offset must be of length 3");
                }
            };

            commands::convert::run(
                kitti_file_path,
                ecoord_file_path,
                *start_date_time,
                *stop_date_time,
                trajectory_channel_id,
                trajectory_frame_id,
                trajectory_child_frame_id,
                world_offset_channel_id,
                world_frame_id,
                world_offset,
            )?;
        }
    };

    Ok(())
}
