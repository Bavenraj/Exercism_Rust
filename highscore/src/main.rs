#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}
impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: Vec::from(scores)
        }
    }
    pub fn scores(&self) -> &[u32] {
        &self.scores[..]
    }
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().copied()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        // had to clone since it was a vector and it takes ownership
        let mut score_copy = self.scores.clone();
        // sort it descending
        score_copy.sort();
        //copy the iterable and reverse it. from this list, retrieve the first three values
        score_copy.iter().copied().rev().take(3).collect()
    }
}

fn main(){
    print!("Testing");
    println("{:?}", HighScores);
}