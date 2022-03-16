use crate::front::ast::*;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn interpret(&mut self, ast: SharedPtr<Ast>) {
        todo!()
    }
}
