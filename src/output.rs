use crate::scorer::ImageScore;
use crate::verdict::Verdict;
use std::path::PathBuf;

// pub struct ScoredImage<'a> {
//     pub path: &'a PathBuf,
//     pub score: &'a ImageScore,
//     pub is_best: bool,
// }

pub fn print_clusters(
    clusters: &[Vec<usize>],
    scored: &[ImageScore],
    paths: &[PathBuf],
) {
    for (cid, cluster) in clusters.iter().enumerate() {
        println!(
            "\nCluster #{} ({} image{})",
            cid,
            cluster.len(),
            if cluster.len() > 1 { "s" } else { "" }
        );

        // Find best image index in this cluster
        let best_idx = cluster
            .iter()
            .max_by(|&&a, &&b| {
                scored[a]
                    .final_score
                    .partial_cmp(&scored[b].final_score)
                    .unwrap()
            })
            .unwrap();

        for &idx in cluster {
            let marker = if idx == *best_idx { "⭐" } else { " " };
            let verdict = match scored[idx].verdict {
                Verdict::Keep => "KEEP",
                Verdict::Maybe => "MAYBE",
                Verdict::Reject => "REJECT",
            };

            println!(
                "  {} {:<20} final={:.2}  {}",
                marker,
                paths[idx]
                    .file_name()
                    .unwrap()
                    .to_string_lossy(),
                scored[idx].final_score,
                verdict
            );
        }
    }
}
