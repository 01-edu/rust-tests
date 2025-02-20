use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Debug)]
pub enum ParseErr {
    Empty,
    Malformed(Box<dyn Error>),
}

impl Display for ParseErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to parse todo file")
    }
}

#[derive(Debug)]
pub struct ReadErr {
    pub child_err: Box<dyn Error>,
}

impl Display for ReadErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to read todo file")
    }
}

impl Error for ParseErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self {
            ParseErr::Empty => None,
            _ => Some(self),
        }
    }
}

impl Error for ReadErr {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.child_err.as_ref())
    }
}
