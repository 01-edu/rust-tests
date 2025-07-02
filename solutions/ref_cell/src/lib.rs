use std::{cell::RefCell, rc::Rc};

pub struct Tracker {
    pub messages: RefCell<Vec<String>>,
    value: RefCell<usize>,
    max: usize,
}

impl Tracker {
    #[inline]
    fn convert_percentage(max: usize, v: usize) -> usize {
        (100 * v) / max
    }

    #[inline]
    pub fn new(max: usize) -> Self {
        Self {
            value: Default::default(),
            max,
            messages: Default::default(),
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        let count = Rc::strong_count(value);
        if count > self.max {
            self.messages
                .borrow_mut()
                .push("Error: You can't go over your quota!".to_owned());
            return;
        }

        self.value.replace(count);
        let percentage_of_max = Self::convert_percentage(self.max, count);

        if percentage_of_max >= 70 {
            self.messages.borrow_mut().push(format!(
                "Warning: You have used up over {percentage_of_max}% of your quota!"
            ));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let percentage_of_max = Self::convert_percentage(self.max, Rc::strong_count(value));
        self.messages.borrow_mut().push(format!(
            "Info: This value would use {percentage_of_max}% of your quota"
        ));
    }
}
