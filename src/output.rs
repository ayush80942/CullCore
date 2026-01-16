use crate::scorer::ImageScore;
use crate::verdict::Verdict;
use std::path::Path;

pub fn print_header() {
    println!(
        "{:<30} {:>6} {:>6} {:>9} {:>11} {:>8}",
        "Image", "Score", "Blur", "Exposure", "Similarity", "Verdict"
    );
    println!("{}", "-".repeat(78));
}

pub fn print_row(path: &Path, score: &ImageScore) {
    let raw = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let name = normalize_name(raw, 30);

    println!(
        "{:<30} {:>6.2} {:>6.2} {:>9.2} {:>11.2} {:>8}",
        name,
        score.final_score,
        score.blur,
        score.exposure,
        score.similarity,
        verdict_str(score.verdict),
    );
}

fn normalize_name(name: &str, max: usize) -> String {
    let cleaned = name
        .replace(' ', "_")
        .replace('\u{202f}', "_"); // narrow no-break space (macOS screenshots)

    if cleaned.len() <= max {
        cleaned
    } else {
        format!("{}…", &cleaned[..max - 1])
    }
}

fn verdict_str(v: Verdict) -> &'static str {
    match v {
        Verdict::Keep => "KEEP",
        Verdict::Maybe => "MAYBE",
        Verdict::Reject => "REJECT",
    }
}
