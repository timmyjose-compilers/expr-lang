use crate::front::source_file::Location;
use std::error::Error;
use std::fmt;

pub type ExprResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct ExprError {
    kind: ExprErrorKind,
    message: String,
}

impl ExprError {
    pub fn new(kind: ExprErrorKind, message: String) -> Self {
        ExprError { kind, message }
    }
}

pub fn report_error(err: ExprError, loc: Option<&Location>) {
    if let Some(loc) = loc {
        eprintln!(
            "[{:?}] line: {}, col: {} - {}",
            loc.source_file, loc.line, loc.col, err
        );
    } else {
        eprintln!("{}", err);
    }
    panic!()
}

#[derive(Debug)]
pub enum ExprErrorKind {
    ScannerError,
    ParserError,
    CheckerError,
    InterpreterError,
}

impl fmt::Display for ExprErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                ExprErrorKind::ScannerError => "Scanner Error",
                ExprErrorKind::ParserError => "Parser Error",
                ExprErrorKind::CheckerError => "Checker Error",
                ExprErrorKind::InterpreterError => "Interpreter Error",
            }
        )
    }
}

impl fmt::Display for ExprError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.kind, self.message)
    }
}

impl Error for ExprError {}
