use crate::front::ast::*;

pub trait Visitor {
    type Result;

    fn visit_assign_expr(&mut self, ass_expr: &AssignExpr) -> Self::Result;
    fn visit_ast(&self, ast: SharedPtr<Ast>) -> Self::Result;
    fn visit_binary_expr(&self, bin_expr: &BinaryExpr) -> Self::Result;
    fn visit_bool_expr(&self, expr: &bool) -> Self::Result;
    fn visit_expr(&self, expr: &Expr) -> Self::Result;
    fn visit_identifier(&self, id: &Identifier) -> Self::Result;
    fn visit_integer_expr(&self, expr: &i32) -> Self::Result;
    fn visit_print_expr(&self, expr: &Expr) -> Self::Result;
    fn visit_unary_expr(&self, un_expr: &UnaryExpr) -> Self::Result;
    fn visit_vname_expr(&self, expr: &VnameExpr) -> Self::Result;
}

pub trait VisitorMut {
    type Result;

    fn visit_assign_expr(&mut self, ass_expr: &mut AssignExpr) -> Self::Result;
    fn visit_ast(&mut self, ast: SharedPtr<Ast>) -> Self::Result;
    fn visit_binary_expr(&mut self, bin_expr: &mut BinaryExpr) -> Self::Result;
    fn visit_bool_expr(&mut self, expr: &mut bool) -> Self::Result;
    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result;
    fn visit_identifier(&mut self, id: &mut Identifier) -> Self::Result;
    fn visit_integer_expr(&mut self, expr: &mut i32) -> Self::Result;
    fn visit_print_expr(&mut self, expr: &mut Expr) -> Self::Result;
    fn visit_unary_expr(&mut self, un_expr: &mut UnaryExpr) -> Self::Result;
    fn visit_vname_expr(&mut self, expr: &mut VnameExpr) -> Self::Result;
}
