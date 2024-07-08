use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about, long_about = None, propagate_version = true)]
pub struct Arguments {
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
}
