#[derive(Debug)]
pub struct HighScores<'a> {
    top: Vec<u32>,
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        let mut top = [0,0,0];
        scores.iter().for_each(
        |n| {
            if *n > top[0] {
                top.swap(1, 2);
                top.swap(0, 1);
                top[0] = *n;
            } else if *n > top[1] {
                top.swap(1, 2);
                top[1] = *n;
            } else if *n > top[2] {
                top[2] = *n;
            }
        });
        let top:Vec<u32> = top.iter().copied().filter(|&n| n > 0).collect();
        HighScores { top, scores }
    }

    #[inline]
    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    #[inline]
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().map(|&e| e)
    }

    #[inline]
    pub fn personal_best(&self) -> Option<u32> {
        self.top.first().map(|&e| e)
    }

    #[inline]
    pub fn personal_top_three(&self) -> Vec<u32> {
        self.top.to_vec()
    }
}
