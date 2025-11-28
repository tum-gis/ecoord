use crate::ecoord::documents::{FrameSerde, TransformEdgeSerde, TransformTreeSerde};
use crate::error::Error;
use ecoord_core::TransformTree;
use std::io::Write;

pub fn write_to_json_file<W: Write>(
    writer: W,
    pretty: bool,
    transform_tree: &TransformTree,
) -> Result<(), Error> {
    let edges_serde: Vec<TransformEdgeSerde> = transform_tree
        .edges
        .values()
        .cloned()
        .map(Into::into)
        .collect();
    let frames_serde: Vec<FrameSerde> = transform_tree
        .frames
        .values()
        .cloned()
        .map(Into::into)
        .collect();

    let transform_tree_serde = TransformTreeSerde {
        edges: edges_serde,
        frames: frames_serde,
    };

    if pretty {
        serde_json::to_writer_pretty(writer, &transform_tree_serde)?;
    } else {
        serde_json::to_writer(writer, &transform_tree_serde)?;
    }

    Ok(())
}
