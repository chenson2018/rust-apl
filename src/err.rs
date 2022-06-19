use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct AplError {
    line: usize,
    err: String,
    lower: Option<io::Error>,
}

impl Error for AplError {
    fn description(&self) -> &str {
        &self.err
    }

    fn cause(&self) -> Option<&dyn Error> {
        if let Some(ref err) = self.lower {
            Some(err as &dyn Error)
        } else {
            None
        }
    }
}

impl fmt::Display for AplError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Error: {} in line {}", &self.err, &self.line)
    }
}

impl AplError {
    pub fn new(s: String, l: usize) -> AplError {
        AplError {
            line: l,
            err: s,
            lower: None,
        }
    }

    pub fn with_lower(s: String, l: usize, e: io::Error) -> AplError {
        AplError {
            line: l,
            err: s,
            lower: Some(e),
        }
    }
}
