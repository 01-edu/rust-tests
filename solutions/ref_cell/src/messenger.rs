pub use std::cell::RefCell;
pub use std::rc::Rc;

pub trait Logger {
    fn warning(&self, msg: &str);
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

#[derive(Debug, Clone)]
pub struct Tracker<'a, T: Logger> {
    logger: &'a T,
    value: RefCell<usize>,
    max: usize,
}

impl<'a, T> Tracker<'a, T>
where
    T: Logger,
{
    pub fn new(logger: &T, max: usize) -> Tracker<T> {
        Tracker {
            logger,
            value: RefCell::new(0),
            max,
        }
    }

    pub fn set_value(&self, value: &Rc<usize>) {
        self.value.replace(Rc::strong_count(value));
        let percentage_of_max = convert_percentage(self.max, Rc::strong_count(&value));

        if percentage_of_max >= 100 {
            self.logger.error("Error: you are over your quota!");
            return;
        } else if percentage_of_max >= 70 {
            self.logger
                .warning(&format!("Warning: you have used up over {percentage_of_max}% of your quota! Proceeds with precaution"));
        }
    }

    pub fn peek(&self, value: &Rc<usize>) {
        let percentage_of_max = convert_percentage(self.max, Rc::strong_count(&value));
        self.logger.info(&format!(
            "Info: you are using up to {percentage_of_max}% of your quota"
        ))
    }
}

fn convert_percentage(max: usize, v: usize) -> usize {
    (100 * v) / max
}
