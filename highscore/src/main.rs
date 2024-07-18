#[derive(Debug)]
pub struct HighScores {
    score: Vec<u32>
}
impl HighScores {
    pub fn new(score: &[u32]) -> Self {
        HighScores {
            score: Vec::from(score)
        }
    }
    pub fn scores(&self) -> &[u32] {
        &self.score[..]
    }
    pub fn latest(&self) -> Option<u32> {
        self.score.last().copied()
    }
    pub fn personal_best(&self) -> Option<u32> {
        self.score.iter().max().copied()
    }
    pub fn personal_top_three(&self) -> Vec<u32> {
        // had to clone since it is a vector and it takes ownership
        let mut score_copy = self.score.clone();
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