// Create an Iterator (by implementing the std::iter::Iterator trait)
// that iterates through the values from beg to end (including end) in
// the indicated steps

// The name of you're iterator will be StepIterator and it must be
// generic so you can use any integer value: i8,..,i64 of floating
// point number f32,..,f64

// Define the associated function: `new` that creates a new Step iterator:
// fn new(beg: T, end: T, step: T) -> StepIterator

// For example:
// for v in StepIterator::new(0, 100, 10) {
// 	println!("{}", v);
// }
// must print:
// 0
// 10
// 20
// 30
// 40
// 50
// 60
// 70
// 80
// 90
// 100

// If the steps don't allow to arrive until the end of the sequence
// only the last value inferior than the end of the series will be returned
// Like beg: 0, end: 100, steps: 7 the last number returned will be
// the last number returned will be 98

pub struct StepIterator<T> {
    beg: T,
    started: bool,
    end: T,
    step: T,
}

use std::ops::Add;
impl<T: Copy + Add<Output = T> + PartialOrd + PartialEq> StepIterator<T> {
    pub fn new(beg: T, end: T, step: T) -> Self {
        Self {
            beg,
            started: false,
            end,
            step,
        }
    }
}

impl<T: PartialEq + Add<Output = T> + PartialOrd + Copy> std::iter::Iterator for StepIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.beg > self.end || self.beg + self.step > self.end {
            return None;
        }
        if !self.started {
            self.started = true;
            return Some(self.beg);
        }
        self.beg = self.beg + self.step;
        Some(self.beg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next() {
        let mut step_iterator = StepIterator::new(0, 100, 10);
        assert_eq!(step_iterator.next(), Some(0));
        assert_eq!(step_iterator.next(), Some(10));
    }

    #[test]
    fn until_the_end() {
        for (i, v) in StepIterator::new(0, 100, 10).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i * 10, v);
        }
    }

    #[test]
    fn test_with_floats() {
        for (i, v) in StepIterator::new(0.0, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5, v);
        }
    }

    #[test]
    fn test_with_floats_with_imperfect_range() {
        for (i, v) in StepIterator::new(0.3, 10.0, 0.5).enumerate() {
            println!("position: {}, value: {}, ", i, v);
            assert_eq!(i as f64 * 0.5 + 0.3, v);
        }
    }
}
