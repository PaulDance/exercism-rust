#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|n| *n)
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().map(|n| *n)
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores_sorted = self.scores.to_vec();
        scores_sorted.sort();
        scores_sorted.iter().rev().take(3).map(|n| *n).collect()
    }
}
