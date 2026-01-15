mod cli;
mod loader;
mod error;
mod metrics;
mod scorer;
mod verdict;

use clap::Parser;
use cli::Cli;
use loader::load_images_from_dir;
use scorer::score_images;
use rayon::ThreadPoolBuilder;

fn main() {
    let cli = Cli::parse();

    if let Some(n) = cli.threads {
        ThreadPoolBuilder::new()
            .num_threads(n)
            .build_global()
            .expect("Failed to set thread pool");
    }

    let loaded = load_images_from_dir(&cli.input)
        .expect("Failed to load images");

    let images: Vec<_> = loaded.iter().map(|i| i.image.clone()).collect();
    let scores = score_images(&images);

    for (i, s) in scores.iter().enumerate() {
        if s.final_score < cli.min_score {
            continue;
        }

        println!(
            "{:?} | blur {:.2} exp {:.2} sim {:.2} => {:.2} {:?}",
            loaded[i].path,
            s.blur,
            s.exposure,
            s.similarity,
            s.final_score,
            s.verdict
        );
    }

    if cli.verbose {
        eprintln!("Processed {} images", scores.len());
    }
}