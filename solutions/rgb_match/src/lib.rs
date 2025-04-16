#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        [&mut self.r, &mut self.g, &mut self.b, &mut self.a]
            .into_iter()
            .for_each(|v| {
                if *v == first {
                    *v = second
                } else if *v == second {
                    *v = first;
                }
            });

        self
    }
}
