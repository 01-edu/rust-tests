use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy> Garage<T> {
    #[inline]
    pub fn move_to_right(&mut self) {
        self.left
            .take()
            .map(|l| self.right = Some(self.right.take().map_or(l, |r| r + l)));
    }

    #[inline]
    pub fn move_to_left(&mut self) {
        self.right
            .take()
            .map(|r| self.left = Some(self.left.take().map_or(r, |l| l + r)));
    }
}
