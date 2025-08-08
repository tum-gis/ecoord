use std::path::Path;

use crate::error::Error;
use tracing::info;

pub fn run(ecoord_file_path: impl AsRef<Path>) -> Result<(), Error> {
    info!("Ecoord path: {}", &ecoord_file_path.as_ref().display());

    let reference_frames = ecoord::io::EcoordReader::from_path(ecoord_file_path)?.finish()?;

    for ((current_channel_id, current_transform_id), current_transforms) in
        reference_frames.transforms()
    {
        info!(
            "channel id: {} transform_id: {}",
            current_channel_id, current_transform_id
        );

        for current_transform in current_transforms
            .iter()
            .enumerate()
            .collect::<Vec<_>>()
            .windows(2)
        {
            let current_timestamp = current_transform[0]
                .1
                .timestamp
                .to_string()
                .replace("UTC", "+0000");
            let end_timestamp = current_transform[1]
                .1
                .timestamp
                .to_string()
                .replace("UTC", "+0000");

            info!(
                "\tTransform {}: --start-date-time \"{}\" --stop-date-time \"{}\"",
                current_transform[0].0, current_timestamp, end_timestamp
            );
        }
        info!(
            "\tTransform {}: --start-date-time \"{}\"",
            current_transforms.len() - 1,
            current_transforms.last().unwrap().timestamp
        );
        info!("");
        //reference_frames.transforms()
    }

    Ok(())
}
