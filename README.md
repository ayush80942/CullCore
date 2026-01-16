# 📸 CullCore  
### High-Performance Photo Culling Engine in Rust

CullCore is a **fast, explainable, and parallel photo culling engine** written in Rust.  
It analyzes images using **sharpness, exposure, and perceptual similarity**, groups redundant photos into clusters, and selects the **best image per burst**.

CullCore is built as an **engine-first system**: today a CLI tool, tomorrow an API or desktop integration.

---

## ✨ Why CullCore?

Professional photographers often shoot **bursts** of near-identical photos.  
The real challenge is not detecting duplicates — it's **choosing the best shot**.

CullCore solves this by:
- Grouping visually similar images into **clusters**
- Ranking images **by quality**, not similarity
- Selecting **one clear winner per cluster**

No machine learning.  
No black boxes.  
Just fast, explainable algorithms.

---

## 🔍 Core Features

- ⚡ **Parallel image processing** using Rayon
- 🔍 **Blur detection** via Laplacian variance
- 🌗 **Exposure analysis** via luminance histogram
- 🧬 **Duplicate detection** using perceptual hashing (pHash)
- 🧠 **Clustering-based similarity handling** (no double penalties)
- 🖥️ **Production-grade CLI** with configurable thresholds
- 📊 **Benchmarked performance** using Criterion

---

## 🧠 How CullCore Works

CullCore follows a **two-phase pipeline**:

### Phase 1 — Similarity (Structure)
Images are grouped into clusters based on perceptual similarity.

> Similarity defines **redundancy**, not quality.

### Phase 2 — Quality (Decision)
Within each cluster:
- Images are ranked by sharpness and exposure
- The best image is selected
- Remaining images are penalized as duplicates

---

## 🧮 Scoring Model (v2)

### Quality Score
```
quality_score = 
  0.55 × blur + 
  0.45 × exposure
```

### Cluster Factor
| Role | Factor |
|----|-------|
| Best in cluster | 1.0 |
| Duplicate | 0.25 |

### Final Score
```
final_score = 
  0.70 × quality_score + 
  0.30 × cluster_factor
```

### Verdicts
| Score | Verdict |
|-----|--------|
| ≥ 0.75 | KEEP |
| 0.50 – 0.74 | MAYBE |
| < 0.50 | REJECT |

---

## 🚀 Usage
```bash
cullcore <IMAGE_DIR> [OPTIONS]
```

### Example
```bash
cullcore /Users/ayush/photos \
  --threads 8 \
  --cluster-threshold 0.90 \
  --min-score 0.75 \
  --verbose
```

---

## 🧩 CLI Options
```
ARGS:
  <INPUT>                    Directory containing images

OPTIONS:
  --threads <N>              Limit Rayon worker threads
  --cluster-threshold <F>    Similarity threshold for clustering (0.0–1.0)
                             Default: 0.90
  --min-score <F>            Minimum final score to display
                             Default: 0.75
  --json <FILE>              Export results as JSON (planned)
  --verbose                  Print processing statistics
```

---

## 👀 Sample Output
```
Cluster #0 (3 images)
  ⭐ IMG_1023.jpg   final=0.91  KEEP
    IMG_1024.jpg   final=0.42  REJECT
    IMG_1025.jpg   final=0.38  REJECT

Cluster #1 (1 image)
  ⭐ IMG_1031.jpg   final=0.88  KEEP
```

⭐ = Best image in cluster

---

## 🏗 Architecture
```
src/
 ├── cli.rs          # CLI interface (Clap)
 ├── loader.rs       # Image loading & validation
 ├── clustering.rs  # Similarity-based clustering
 ├── metrics/
 │    ├── blur.rs
 │    ├── exposure.rs
 │    └── similarity.rs
 ├── scorer.rs      # Parallel scoring engine
 ├── verdict.rs     # KEEP / MAYBE / REJECT logic
 ├── output.rs      # CLI visualization
 └── main.rs
```

### Design Principles

* Engine-first architecture
* Deterministic & explainable logic
* Zero shared mutable state
* Parallelism without locks

---

## 🧠 Why Rust?

* Zero-cost abstractions
* Memory safety without GC
* Fearless parallelism
* Predictable performance for CPU-heavy image pipelines

---

## 🛣 Roadmap (v3)

* Axum-based REST API
* JSON / CSV exports
* Smarter cluster optimization (LSH / graph components)
* Adaptive thresholds per photo set
* Desktop / TUI visualization

---

## 🧑‍💻 Author

Built by **Ayush Aggarwal**
For Rust Delhi & systems-level Rust exploration.

---

## 📜 License

MIT
