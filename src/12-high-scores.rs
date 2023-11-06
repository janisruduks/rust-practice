fn main() {
    // u32 are only positive numbers
    // Best way to deal with options is just to pass them, makes code cleaner and better to read
    let scores = vec![100, 0, 90, 30];
    let highscores = HighScores::new(scores);
    assert_eq!(highscores.latest(), Some(30));
    assert_eq!(highscores.best(), Some(100));
    assert_eq!(highscores.top_three_scores(), vec![100, 90, 30]);
    println!("Test with valid scores completed");
    let x = HighScores::new(vec![]);
    assert_eq!(x.latest(), None);
    assert_eq!(x.best(), None);
    assert_eq!(x.top_three_scores(), vec![]);
    println!("Test with invalid values completed");
}

struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    fn new(scores: Vec<u32>) -> Self {
        HighScores { scores }
    }

    fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    fn best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    fn top_three_scores(&self) -> Vec<u32> {
        let mut highscores = self.scores.clone();
        highscores.sort_by(|a, b| b.cmp(a));
        highscores.iter().take(3).cloned().collect()
    }
}
