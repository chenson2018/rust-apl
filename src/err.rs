use std::error::Error;
use std::fmt;
use std::io;

#[derive(Debug)]
pub struct AplError {
    line: usize,
    pub err: String,
    pub message: String,
    pub label: String,
    lower: Option<io::Error>,
    pub start: usize,
    pub end: usize,
}

#[derive(Debug)]
pub struct ErrCtx {
    pub err: String,
    pub message: String,
    pub label: String,
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

#[derive(Debug)]
pub struct AplErrors(pub Vec<AplError>);

impl fmt::Display for AplErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, error| {
            result.and_then(|_| writeln!(f, "{}", error))
        })
    }
}

impl AplError {
    pub fn with_pos(
        s: String,
        l: usize,
        start: usize,
        end: usize,
        label: String,
        message: String,
    ) -> AplError {
        AplError {
            line: l,
            err: s,
            lower: None,
            start,
            end,
            label,
            message,
        }
    }
}
