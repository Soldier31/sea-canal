pub struct Stepper {
    start: usize,
    end: usize,
    step: usize
}

impl Stepper {
    pub fn new(start: usize, end: usize, step: usize) -> Self {
        Stepper { start: start, end: end, step: step }
    }
}

impl Iterator for Stepper {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.start >= self.end {
            return None;
        }

        let next = self.start;
        self.start += self.step;

        Some(next)
    }
}

#[macro_export]
macro_rules! step {
    ($start:expr => $end:expr; $step:expr) => { Stepper::new($start, $end, $step).into_iter() }
}
