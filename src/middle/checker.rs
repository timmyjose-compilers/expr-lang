use crate::front::ast::*;

pub struct Checker;

impl Checker {
    pub fn new() -> Self {
        Checker
    }

    pub fn check(&mut self, ast: SharedPtr<Ast>) {}
}
