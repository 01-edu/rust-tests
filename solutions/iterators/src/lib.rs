#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Collatz {
    pub fn new(aux: u64) -> Self {
        Collatz { v: aux }
    }
}

impl Iterator for Collatz {
    type Item = Collatz;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v <= 1 {
            None
        } else {
            let old_value = self.v;
            if self.v % 2 == 0 {
                self.v /= 2;
            } else {
                self.v = self.v * 3 + 1;
            }
            Some(Collatz { v: old_value })
        }
    }
}

pub fn collatz(n: u64) -> usize {
    let nb = Collatz::new(n);
    nb.count()
}
