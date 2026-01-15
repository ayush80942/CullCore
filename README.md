# 📸 CullCore

**High-Performance Photo Culling Engine in Rust**

CullCore is a fast, explainable, and parallel photo culling engine written in Rust. It analyzes image quality signals—sharpness, exposure, and similarity—to automatically rank and classify photos as KEEP / MAYBE / REJECT.

Built with real-world photography workflows in mind.

## ✨ Features

* ⚡ Parallel image scoring using Rayon
* 🔍 Blur detection via Laplacian variance
* 🌗 Exposure analysis via luminance histogram
* 🧬 Duplicate detection using perceptual hashing (pHash)
* 🧠 Explainable weighted scoring (no ML black boxes)
* 🖥️ Production-grade CLI with configurable threads
* 📊 Benchmarked performance using Criterion

## 🧠 How It Works

Each image is scored on three normalized signals:

| Signal     | Method                    | Why                              |
|------------|---------------------------|----------------------------------|
| Blur       | Laplacian variance        | Detects loss of edge detail      |
| Exposure   | Luminance histogram       | Penalizes clipped blacks/whites  |
| Similarity | Perceptual hash (pHash)   | Detects burst duplicates         |

### Final Score
```
final_score =
  0.45 × blur +
  0.35 × exposure +
  0.20 × similarity
```

### Verdicts

| Score        | Verdict |
|--------------|---------|
| ≥ 0.75       | KEEP    |
| 0.50–0.74    | MAYBE   |
| < 0.50       | REJECT  |

## 🚀 Usage
```bash
cullcore ./images --threads 8 --min-score 0.75 --verbose
```

### CLI Options
```
ARGS:
  <INPUT>              Directory containing images

OPTIONS:
  --threads <N>         Limit Rayon worker threads
  --min-score <FLOAT>   Filter images below this score (default: 0.75)
  --json <FILE>         Export results as JSON (v2)
  --verbose             Print processing stats
```

## ⚡ Performance

Benchmarked using `criterion` (Release mode):

| Task                    | Result                  |
|-------------------------|-------------------------|
| Blur (1MP image)        | ~X ms                   |
| Exposure (1MP)          | ~Y ms                   |
| 50 images scoring       | ~Z ms (parallel)        |

CullCore scales linearly per image and parallelizes cleanly across CPU cores.

## 🧱 Architecture
```
src/
 ├── cli.rs        # CLI interface (Clap)
 ├── loader.rs     # Image loading & validation
 ├── metrics/
 │    ├── blur.rs
 │    ├── exposure.rs
 │    └── similarity.rs
 ├── scorer.rs    # Parallel scoring engine
 ├── verdict.rs   # KEEP / MAYBE / REJECT logic
 └── main.rs
```

Core engine is interface-agnostic → CLI today, Axum API tomorrow.