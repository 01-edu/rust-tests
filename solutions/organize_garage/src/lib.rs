use std::ops::Add;

#[derive(Debug, PartialEq, Eq)]
pub struct Garage<T> {
    pub left: Option<T>,
    pub right: Option<T>,
}

impl<T: Add<Output = T> + Copy> Garage<T> {
    pub fn move_to_right(&mut self) {
        if let Some(left_stuff) = self.left {
            match self.right {
                Some(right_stuff) => self.right = Some(right_stuff + left_stuff),
                _ => self.right = Some(left_stuff),
            }
            self.left = None;
        }
    }
    pub fn move_to_left(&mut self) {
        if let Some(right_stuff) = self.right {
            match self.left {
                Some(left_stuff) => self.left = Some(left_stuff + right_stuff),
                _ => self.left = Some(right_stuff),
            }
            self.right = None;
        }
    }
}
