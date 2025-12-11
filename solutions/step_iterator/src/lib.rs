use std::ops::{Add, AddAssign};

pub struct StepIterator<T: PartialOrd + Add<Output = T> + AddAssign + Copy + Clone> {
    started: bool,
    curr: T,
    end: T,
    step: T,
}

impl<T: PartialOrd + Add<Output = T> + AddAssign + Copy + Clone> StepIterator<T> {
    pub const fn new(beg: T, end: T, step: T) -> Self {
        Self {
            started: false,
            curr: beg,
            end,
            step,
        }
    }
}

impl<T: PartialOrd + Add<Output = T> + AddAssign + Copy + Clone> Iterator for StepIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.end || self.curr + self.step > self.end {
            return None;
        }

        if self.started {
            self.curr += self.step;
        } else {
            self.started = true;
        }

        Some(self.curr)
    }
}
