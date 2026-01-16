mod cli;
mod error;
mod loader;
mod metrics;
mod output;
mod scorer;
mod verdict;

use clap::Parser;
use cli::Cli;
use loader::load_images_from_dir;
use output::{print_header, print_row};
use rayon::ThreadPoolBuilder;
use scorer::score_images;

fn main() {
    let cli = Cli::parse();

    if let Some(n) = cli.threads {
        ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .expect("Failed to set thread pool");
    }

    let loaded = load_images_from_dir(&cli.input).expect("Failed to load images");

    let images: Vec<_> = loaded.iter().map(|i| i.image.clone()).collect();
    let scores = score_images(&images);

    print_header();

    for (i, s) in scores.iter().enumerate() {
        if s.final_score < cli.min_score {
            continue;
        }

        print_row(&loaded[i].path, s);
    }

    if cli.verbose {
        eprintln!("Processed {} images", scores.len());
    }
}
