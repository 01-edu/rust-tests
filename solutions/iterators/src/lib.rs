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
        let old_value = self.v;

		if self.v <= 1
		{
			return None;
		} else {
            if self.v % 2 == 0 {
                self.v = self.v / 2;
            } else {
                self.v = self.v * 3 + 1;
            }
            return Some(Collatz { v: old_value });
        }
	}
}


pub fn collatz(n: u64) -> Option<u64> {
    let nb = Collatz::new(n);
	Some(nb.into_iter().count() as u64)
}
