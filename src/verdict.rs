#[derive(Debug, Clone, Copy)]
pub enum Verdict {
    Keep,
    Maybe,
    Reject,
}

impl Verdict {
    pub fn from_score(score: f32) -> Self {
        match score {
            s if s >= 0.75 => Verdict::Keep,
            s if s >= 0.50 => Verdict::Maybe,
            _ => Verdict::Reject,
        }
    }
}
