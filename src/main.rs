use expr_lang::backend::interpreter::Interpreter;
use expr_lang::front::{parser::Parser, scanner::Scanner, source_file::SourceFile};
use expr_lang::middle::checker::Checker;
use std::rc::Rc;

use std::env;

fn main() {
    println!("Welcome to expr lang");

    let args = env::args().skip(1).collect::<Vec<String>>();
    if args.len() != 1 {
        usage();
    }

    let source_file_path = &args[0];
    let source_file = SourceFile::new(source_file_path);

    let mut scanner = Scanner::new(source_file.chars);
    scanner.scan_all();

    for token in &scanner.tokens {
        println!("{:?}", token);
    }

    let mut parser = Parser::new(scanner.tokens);
    let ast = parser.parse();
    println!("{:#?}", ast);

    let mut checker = Checker::new();
    checker.check(Rc::clone(&ast));
    println!("{:#?}", ast);

    let mut interpreter = Interpreter::new();
    interpreter.interpret(Rc::clone(&ast));
}

fn usage() {
    eprintln!("USAGE: expr <source-file>");
    std::process::exit(1);
}
