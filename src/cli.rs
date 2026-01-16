use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "CullCore",
    version,
    about = "High-performance photo culling engine written in Rust"
)]
pub struct Cli {
    pub input: PathBuf,

    #[arg(long)]
    pub threads: Option<usize>,

    /// Similarity threshold for clustering (0.0–1.0)
    #[arg(long, default_value_t = 0.90)]
    pub cluster_threshold: f32,

    /// Minimum final score to show
    #[arg(long, default_value_t = 0.75)]
    pub min_score: f32,

    #[arg(long)]
    pub json: Option<PathBuf>,

    #[arg(long)]
    pub verbose: bool,
}