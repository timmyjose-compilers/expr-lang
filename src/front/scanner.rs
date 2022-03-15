use super::source_file::SourceFile;
use super::token::Token;
use std::path::PathBuf;

pub struct Scanner<P> {
    source_file: SourceFile<P>,
}

impl<P> Scanner<P>
where
    P: Into<PathBuf>,
{
    pub fn new(source_file: SourceFile<P>) -> Self {
        Scanner { source_file }
    }

    pub fn scan_all(&mut self) -> Vec<Token> {
        vec![]
    }
}
