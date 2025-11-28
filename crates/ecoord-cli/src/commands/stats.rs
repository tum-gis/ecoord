use crate::error::Error;
use ecoord::{DynamicTransform, TransformEdge};
use std::path::Path;
use tracing::info;

pub fn run(ecoord_file_path: impl AsRef<Path>) -> Result<(), Error> {
    info!("Ecoord path: {}", &ecoord_file_path.as_ref().display());

    let transform_tree = ecoord::io::EcoordReader::from_path(ecoord_file_path)?.finish()?;

    for (current_transform_id, current_edge) in transform_tree.edges() {
        info!("transform_id: {}", current_transform_id);

        match current_edge {
            TransformEdge::Static(_) => {}
            TransformEdge::Dynamic(x) => print_dynamic_transform(x),
        }

        info!("");
    }

    Ok(())
}

fn print_dynamic_transform(dynamic_transform: &DynamicTransform) {
    for current_transform in dynamic_transform
        .samples
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
        dynamic_transform.samples.len() - 1,
        dynamic_transform.samples.last().unwrap().timestamp
    );
}
