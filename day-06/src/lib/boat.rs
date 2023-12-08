#[derive(Debug)]
pub struct Race {
    pub best_time: usize,
    pub distance: usize
}

impl Race {
    #[must_use]
    pub fn num_winners(&self) -> usize {
        (0..self.best_time).filter(|speed| {
            let race_time = self.best_time - speed;
            speed * race_time > self.distance
        }).count()
    }
}