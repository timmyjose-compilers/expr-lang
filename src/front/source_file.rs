use crate::error::{report_error, ExprError, ExprErrorKind, ExprResult};
use std::default::Default;
use std::fmt::Debug;
use std::fs;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

pub const NUL: char = '\u{0}';

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub source_file: PathBuf,
    pub line: usize,
    pub col: usize,
}

impl Location {
    pub fn new(source_file: PathBuf, line: usize, col: usize) -> Self {
        Location {
            source_file,
            line,
            col,
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Location {
            source_file: PathBuf::default(),
            line: usize::default(),
            col: usize::default(),
        }
    }
}

#[derive(Debug)]
pub struct Char {
    pub c: char,
    pub loc: Location,
}

impl Char {
    pub fn new(c: char, loc: Location) -> Self {
        Char { c, loc }
    }
}

#[derive(Debug)]
pub struct SourceFile {
    pub chars: Vec<Char>,
}

impl SourceFile {
    pub fn new<P: AsRef<Path> + Debug>(source_file_path: P) -> Self {
        let chars = SourceFile::init_source_file(&source_file_path);

        if chars.is_err() {
            report_error(
                ExprError::new(
                    ExprErrorKind::ScannerError,
                    format!("Could not open source file {:?}", source_file_path),
                ),
                None,
            );
        }

        SourceFile {
            chars: chars.unwrap(),
        }
    }

    fn init_source_file<P: AsRef<Path>>(source_file_path: &P) -> ExprResult<Vec<Char>> {
        let mut source_reader = io::BufReader::new(fs::File::open(source_file_path)?);
        let mut file_contents = String::new();
        source_reader.read_to_string(&mut file_contents)?;
        file_contents.push(NUL); // for eof

        let mut line = 1;
        let mut col = 1;

        let chars = file_contents
            .chars()
            .map(|c| {
                let ch = Char::new(
                    c,
                    Location::new(source_file_path.as_ref().to_owned(), line, col),
                );

                if c == '\n' {
                    line += 1;
                    col = 1;
                } else {
                    col += 1;
                }

                ch
            })
            .collect::<Vec<Char>>();

        Ok(chars)
    }
}
