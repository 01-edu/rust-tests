use std::fmt;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum ParseErr {
    Malformed(Box<dyn Error>),
    Empty
}

// required by error trait
impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to parses todo")
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>
}

// required by error trait
impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fail to read todo file")
    }
}

impl Error for ReadErr {
    fn description(&self) -> &str {
        "Todo List read failed: "
    }
    fn cause(&self) -> Option<&dyn Error> {
        Some(&*self.child_err)
    }
}

impl Error for ParseErr {
    fn description(&self) -> &str {
        "Todo List parse failed: "
    }
    fn cause(&self) -> Option<&dyn Error> {
        match &self {
            ParseErr::Empty => None,
            _ => Some(&*self)
        }
    }
}
