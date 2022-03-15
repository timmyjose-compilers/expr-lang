use crate::front::ast::Ast;

pub struct Checker;

impl Checker {
    pub fn new() -> Self {
        Checker
    }

    pub fn check(&mut self, ast: &mut Ast) {}
}
