use super::runtime::{ExprValue, Runtime};
use crate::front::ast::*;
use crate::middle::visitor::VisitorMut;

pub struct Interpreter {
    runtime: Runtime,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            runtime: Runtime::new(),
        }
    }

    pub fn interpret(&mut self, ast: &mut Ast) {
        match self.visit_ast(ast) {
            ExprValue::Int(ival) => println!("{}", ival),
            ExprValue::Float(fval) => println!("{}", fval),
            ExprValue::Bool(bval) => println!("{}", bval),
            ExprValue::None => {}
        }
    }
}

impl VisitorMut for Interpreter {
    type Result = ExprValue;

    fn visit_assign_expr(&mut self, ass_expr: &mut AssignExpr) -> Self::Result {
        let var_name_val = self.visit_expr(&mut ass_expr.vname);
        let var_val = self.visit_expr(&mut ass_expr.expr);

        if let Expr::VnameExpr(ref vname_expr) = *ass_expr.vname {
            match ass_expr.op {
                BinaryOperator::Assign => {
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, var_val.clone());
                    var_val
                }

                BinaryOperator::AddAssign => {
                    let sum_val = var_name_val + var_val;
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, sum_val.clone());
                    sum_val
                }

                BinaryOperator::SubAssign => {
                    let sub_val = var_name_val.clone() - var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, sub_val.clone());
                    sub_val
                }

                BinaryOperator::MulAssign => {
                    let mul_val = var_name_val.clone() * var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, mul_val.clone());
                    mul_val
                }

                BinaryOperator::DivAssign => {
                    let div_val = var_name_val.clone() / var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, div_val.clone());
                    div_val
                }

                BinaryOperator::ModAssign => {
                    let mod_val = var_name_val.clone() % var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, mod_val.clone());
                    mod_val
                }

                BinaryOperator::BitwiseAndAssign => {
                    let bitand_val = var_name_val.clone() & var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, bitand_val.clone());
                    bitand_val
                }

                BinaryOperator::BitwiseOrAssign => {
                    let bitor_val = var_name_val.clone() | var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, bitor_val.clone());
                    bitor_val
                }
                BinaryOperator::BitwiseXorAssign => {
                    let bitxor_val = var_name_val.clone() ^ var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, bitxor_val.clone());
                    bitxor_val
                }

                BinaryOperator::LeftShiftAssign => {
                    let shl_val = var_name_val.clone() + var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, shl_val.clone());
                    shl_val
                }

                BinaryOperator::RightShiftAssign => {
                    let shr_val = var_name_val.clone() >> var_val.clone();
                    self.runtime
                        .save_binding(&vname_expr.id.spelling, shr_val.clone());
                    shr_val
                }

                _ => unreachable!(),
            }
        } else {
            ExprValue::None
        }
    }

    fn visit_ast(&mut self, ast: &mut Ast) -> Self::Result {
        let mut resp = ExprValue::None;

        for expr in &mut ast.exprs {
            resp = self.visit_expr(expr);
        }

        resp
    }

    fn visit_binary_expr(&mut self, bin_expr: &mut BinaryExpr) -> Self::Result {
        let lhs_val = self.visit_expr(&mut bin_expr.lhs);
        let rhs_val = self.visit_expr(&mut bin_expr.rhs);

        match bin_expr.op {
            BinaryOperator::Add => lhs_val + rhs_val,
            BinaryOperator::BitwiseAnd => lhs_val & rhs_val,
            BinaryOperator::BitwiseOr => lhs_val | rhs_val,
            BinaryOperator::BitwiseXor => lhs_val ^ rhs_val,
            BinaryOperator::Div => lhs_val / rhs_val,
            BinaryOperator::Equal => ExprValue::Bool(lhs_val == rhs_val),
            BinaryOperator::GreaterThan => ExprValue::Bool(lhs_val > rhs_val),
            BinaryOperator::GreaterThanOrEqual => ExprValue::Bool(lhs_val >= rhs_val),
            BinaryOperator::LeftShift => lhs_val << rhs_val,
            BinaryOperator::LessThan => ExprValue::Bool(lhs_val < rhs_val),
            BinaryOperator::LessThanOrEqual => ExprValue::Bool(lhs_val <= rhs_val),
            BinaryOperator::LogicalAnd => {
                if let ExprValue::Bool(lhs_val) = lhs_val {
                    if let ExprValue::Bool(rhs_val) = rhs_val {
                        ExprValue::Bool(lhs_val && rhs_val)
                    } else {
                        ExprValue::None
                    }
                } else {
                    ExprValue::None
                }
            }
            BinaryOperator::LogicalOr => {
                if let ExprValue::Bool(lhs_val) = lhs_val {
                    if let ExprValue::Bool(rhs_val) = rhs_val {
                        ExprValue::Bool(lhs_val || rhs_val)
                    } else {
                        ExprValue::None
                    }
                } else {
                    ExprValue::None
                }
            }
            BinaryOperator::Mod => lhs_val % rhs_val,
            BinaryOperator::Mul => lhs_val * rhs_val,
            BinaryOperator::NotEqual => ExprValue::Bool(lhs_val == rhs_val),
            BinaryOperator::RightShift => lhs_val >> rhs_val,
            BinaryOperator::Sub => lhs_val - rhs_val,

            _ => unreachable!(),
        }
    }

    fn visit_bool_expr(&mut self, bool_val: &mut bool) -> Self::Result {
        ExprValue::Bool(*bool_val)
    }

    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result {
        match expr {
            Expr::AssignExpr(ref mut ass_expr) => self.visit_assign_expr(ass_expr),
            Expr::BinaryExpr(ref mut bin_expr) => self.visit_binary_expr(bin_expr),
            Expr::BoolExpr(ref mut bool_expr) => self.visit_bool_expr(bool_expr),
            Expr::IntegerExpr(ref mut int_expr) => self.visit_integer_expr(int_expr),
            Expr::PrintExpr(ref mut print_expr) => self.visit_print_expr(print_expr),
            Expr::UnaryExpr(ref mut unary_expr) => self.visit_unary_expr(unary_expr),
            Expr::VnameExpr(ref mut vname_expr) => self.visit_vname_expr(vname_expr),
        }
    }

    fn visit_identifier(&mut self, id: &mut Identifier) -> Self::Result {
        self.runtime.get_binding(&id.spelling)
    }

    fn visit_integer_expr(&mut self, int_val: &mut i32) -> Self::Result {
        ExprValue::Int(*int_val)
    }

    fn visit_print_expr(&mut self, print_expr: &mut Expr) -> Self::Result {
        let expr_val = self.visit_expr(print_expr);
        println!("{}", expr_val);
        ExprValue::None
    }

    fn visit_unary_expr(&mut self, unary_expr: &mut UnaryExpr) -> Self::Result {
        let expr_val = self.visit_expr(&mut unary_expr.elem);

        match unary_expr.op {
            UnaryOperator::UnaryPlus => expr_val,
            UnaryOperator::BitwiseNot => !expr_val,
            UnaryOperator::LogicalNot => !expr_val,
            UnaryOperator::UnaryMinus => -expr_val,
        }
    }

    fn visit_vname_expr(&mut self, vname_expr: &mut VnameExpr) -> Self::Result {
        self.visit_identifier(&mut vname_expr.id)
    }
}
