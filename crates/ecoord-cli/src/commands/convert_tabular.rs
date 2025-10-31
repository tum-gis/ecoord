use crate::error::Error;
use ecoord::io::{
    EcoordWriter, FILE_EXTENSION_ECOORD_FORMAT, FILE_EXTENSION_TABULAR_CSV_FORMAT, TabularReader,
};
use ecoord::{ChannelId, FrameId};
use std::fs;
use std::path::{Path, PathBuf};
use tracing::info;
use walkdir::WalkDir;

pub fn run(
    input_path: impl AsRef<Path>,
    output_path: impl AsRef<Path>,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,

    pretty: bool,
) -> Result<(), Error> {
    info!(
        "Convert from a tabular structure at {}",
        input_path.as_ref().display()
    );
    if input_path.as_ref().is_file() && output_path.as_ref().is_file() {
        process_individual_file(
            input_path,
            output_path,
            trajectory_channel_id,
            trajectory_frame_id,
            trajectory_child_frame_id,
            pretty,
        )?;
    } else if input_path.as_ref().is_dir() && output_path.as_ref().is_dir() {
        process_multiple_files(
            input_path,
            output_path,
            trajectory_channel_id,
            trajectory_frame_id,
            trajectory_child_frame_id,
            pretty,
        )?;
    } else {
        panic!("input_path and output_path must be either both files or both directories");
    }

    Ok(())
}

fn process_multiple_files(
    input_directory_path: impl AsRef<Path>,
    output_directory_path: impl AsRef<Path>,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    pretty: bool,
) -> Result<(), Error> {
    let input_file_paths: Vec<PathBuf> = WalkDir::new(input_directory_path)
        .sort_by_file_name()
        .into_iter()
        .filter_entry(|e| {
            // Skip hidden files and directories
            e.file_name()
                .to_str()
                .map(|s| !s.starts_with('.'))
                .unwrap_or(false)
        })
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap().path().to_owned())
        .filter(|x| {
            x.extension()
                .is_some_and(|ext| ext == FILE_EXTENSION_TABULAR_CSV_FORMAT)
        })
        .collect();
    info!("Total {}", input_file_paths.len());

    for current_input_file_path in input_file_paths {
        let output_file_path = output_directory_path
            .as_ref()
            .join(current_input_file_path.file_stem().unwrap())
            .with_extension(FILE_EXTENSION_ECOORD_FORMAT);

        process_individual_file(
            &current_input_file_path,
            &output_file_path,
            trajectory_channel_id.clone(),
            trajectory_frame_id.clone(),
            trajectory_child_frame_id.clone(),
            pretty,
        )?;
    }

    Ok(())
}

fn process_individual_file(
    input_file_path: impl AsRef<Path>,
    output_file_path: impl AsRef<Path>,
    trajectory_channel_id: ChannelId,
    trajectory_frame_id: FrameId,
    trajectory_child_frame_id: FrameId,
    pretty: bool,
) -> Result<(), Error> {
    let reference_frames = TabularReader::from_path(&input_file_path)?
        .with_trajectory_channel_id(trajectory_channel_id)
        .with_trajectory_frame_id(trajectory_frame_id)
        .with_trajectory_child_frame_id(trajectory_child_frame_id)
        .finish()?;

    fs::create_dir_all(
        output_file_path
            .as_ref()
            .parent()
            .expect("must have a parent"),
    )?;
    EcoordWriter::from_path(&output_file_path)?
        .with_pretty(pretty)
        .finish(&reference_frames)?;
    info!(
        "Completed conversion and writing to {}",
        output_file_path.as_ref().display()
    );

    Ok(())
}
