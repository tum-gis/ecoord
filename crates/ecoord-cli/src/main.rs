mod arguments;
mod commands;

use crate::arguments::{Arguments, Commands};
use clap::Parser;
use std::path::PathBuf;

fn main() {
    tracing_subscriber::fmt::init();
    let arguments = Arguments::parse();

    match &arguments.command {
        Commands::Stats { ecoord_file_path } => {
            let ecoord_file_path = PathBuf::from(ecoord_file_path);

            commands::stats::run(ecoord_file_path);
        }
    };
}
