use std::rc::Rc;

mod common;

use std::io;
use std::path::PathBuf;

use expr_lang::front::{parser::Parser, scanner::Scanner, source_file::SourceFile};
use expr_lang::middle::checker::Checker;

#[test]
fn all_checker_tests() -> io::Result<()> {
    let mut test_files = Vec::new();
    common::get_all_test_files(&"examples", &mut test_files)?;

    for file in test_files {
        run_test(file);
    }

    Ok(())
}

fn run_test(test_file: PathBuf) {
    let mut scanner = Scanner::new(SourceFile::new(test_file).chars);
    scanner.scan_all();
    let mut parser = Parser::new(scanner.tokens);
    let ast = parser.parse();
    let mut checker = Checker::new();
    checker.check(Rc::clone(&ast));
    println!("{:#?}", ast);
}
