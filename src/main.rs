mod cli;
mod clustering;
mod error;
mod loader;
mod metrics;
mod output;
mod scorer;
mod verdict;

use clap::Parser;
use cli::Cli;
use loader::load_images_from_dir;
use output::print_clusters;
use rayon::ThreadPoolBuilder;
use scorer::score_images_with_clustering;

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
    let result = score_images_with_clustering(&images, cli.cluster_threshold);

    print_clusters(
        &result.clusters,
        &result.scores,
        &loaded.iter().map(|i| i.path.clone()).collect::<Vec<_>>(),
    );

    // if cli.verbose {
    //     eprintln!("Processed {} images", scores.len());
    // }
}
