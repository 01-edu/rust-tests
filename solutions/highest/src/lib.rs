#[derive(Debug)]
pub struct Numbers<const N: usize> {
    numbers: [u32; N],
}

impl<const N: usize> Numbers<N> {
    pub const fn new(numbers: [u32; N]) -> Self {
        Self { numbers }
    }

    pub const fn inner(&self) -> &[u32] {
        self.numbers.as_slice()
    }

    pub const fn latest(&self) -> Option<u32> {
        self.numbers.last().copied()
    }

    #[inline]
    pub fn highest(&self) -> Option<u32> {
        self.numbers.iter().max().copied()
    }

    #[inline]
    pub fn highest_three(&self) -> Option<[u32; 3]> {
        if self.numbers.len() < 3 {
            return None;
        }

        let mut ord = self.numbers;
        ord.sort_unstable_by(|a, b| b.cmp(a));

        Some([ord[0], ord[1], ord[2]])
    }
}
