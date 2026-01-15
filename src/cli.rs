use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "CullCore",
    version,
    about = "High-performance photo culling engine written in Rust"
)]
pub struct Cli {
    /// Directory containing images
    pub input: PathBuf,

    /// Number of worker threads (default: logical cores)
    #[arg(long)]
    pub threads: Option<usize>,

    /// Minimum final score to mark image as KEEP
    #[arg(long, default_value_t = 0.75)]
    pub min_score: f32,

    /// Output results as JSON file
    #[arg(long)]
    pub json: Option<PathBuf>,

    /// Enable verbose logging
    #[arg(long)]
    pub verbose: bool,
}
