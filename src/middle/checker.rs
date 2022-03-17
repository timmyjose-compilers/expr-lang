use super::id_table::{DeclOrId, IdentificationTable};
use super::stdenv::{self};
use super::visitor::VisitorMut;
use crate::error::report_error;
use crate::error::*;
use crate::front::ast::*;

pub struct Checker {
    id_table: IdentificationTable,
}

impl Checker {
    pub fn new() -> Self {
        let mut id_table = IdentificationTable::new();
        stdenv::load_stdenv(&mut id_table);
        Checker { id_table }
    }

    pub fn check(&mut self, ast: SharedPtr<Ast>) {
        self.visit_ast(ast);
    }

    fn get_unary_op_spec(&self, op: &UnaryOperator) -> &DeclOrId {
        match *op {
            UnaryOperator::BitwiseNot => self.id_table.get_attr("bitwise_not").unwrap(),
            UnaryOperator::LogicalNot => self.id_table.get_attr("logical_not").unwrap(),
            UnaryOperator::UnaryMinus => self.id_table.get_attr("unary_minus").unwrap(),
            UnaryOperator::UnaryPlus => self.id_table.get_attr("unary_plus").unwrap(),
        }
    }

    fn get_bin_op_spec(&self, op: &BinaryOperator) -> &DeclOrId {
        match *op {
            BinaryOperator::Add => self.id_table.get_attr("add").unwrap(),
            BinaryOperator::AddAssign => self.id_table.get_attr("add_assign").unwrap(),
            BinaryOperator::Assign => self.id_table.get_attr("assign").unwrap(),
            BinaryOperator::BitwiseAnd => self.id_table.get_attr("bitwise_and").unwrap(),
            BinaryOperator::BitwiseAndAssign => {
                self.id_table.get_attr("bitwise_and_assign").unwrap()
            }
            BinaryOperator::BitwiseOr => self.id_table.get_attr("bitwise_or").unwrap(),
            BinaryOperator::BitwiseOrAssign => self.id_table.get_attr("bitwise_or_assign").unwrap(),
            BinaryOperator::BitwiseXor => self.id_table.get_attr("bitwise_xor").unwrap(),
            BinaryOperator::BitwiseXorAssign => {
                self.id_table.get_attr("bitwise_xor_assign").unwrap()
            }
            BinaryOperator::Div => self.id_table.get_attr("div").unwrap(),
            BinaryOperator::DivAssign => self.id_table.get_attr("div_assign").unwrap(),
            BinaryOperator::Equal => self.id_table.get_attr("equal").unwrap(),
            BinaryOperator::GreaterThan => self.id_table.get_attr("greater_than").unwrap(),
            BinaryOperator::GreaterThanOrEqual => {
                self.id_table.get_attr("greater_than_or_equal").unwrap()
            }
            BinaryOperator::LeftShift => self.id_table.get_attr("left_shift").unwrap(),
            BinaryOperator::LeftShiftAssign => self.id_table.get_attr("left_shift_assign").unwrap(),
            BinaryOperator::LessThan => self.id_table.get_attr("less_than").unwrap(),
            BinaryOperator::LessThanOrEqual => {
                self.id_table.get_attr("less_than_or_equal").unwrap()
            }
            BinaryOperator::LogicalAnd => self.id_table.get_attr("logical_and").unwrap(),
            BinaryOperator::LogicalAndAssign => {
                self.id_table.get_attr("logical_and_assign").unwrap()
            }
            BinaryOperator::LogicalOr => self.id_table.get_attr("logical_or").unwrap(),
            BinaryOperator::LogicalOrAssign => self.id_table.get_attr("logical_or_assign").unwrap(),
            BinaryOperator::Mod => self.id_table.get_attr("mod").unwrap(),
            BinaryOperator::ModAssign => self.id_table.get_attr("mod_assign").unwrap(),
            BinaryOperator::Mul => self.id_table.get_attr("mul").unwrap(),
            BinaryOperator::MulAssign => self.id_table.get_attr("mul_assign").unwrap(),
            BinaryOperator::NotEqual => self.id_table.get_attr("not_equal").unwrap(),
            BinaryOperator::RightShift => self.id_table.get_attr("right_shift").unwrap(),
            BinaryOperator::RightShiftAssign => {
                self.id_table.get_attr("right_shift_assign").unwrap()
            }
            BinaryOperator::Sub => self.id_table.get_attr("sub").unwrap(),
            BinaryOperator::SubAssign => self.id_table.get_attr("sub_assign").unwrap(),
        }
    }
}

impl VisitorMut for Checker {
    type Result = Option<Type>;

    /// Type-check ast:
    /// - type-check all the expts in the ast.
    fn visit_ast(&mut self, ast: SharedPtr<Ast>) -> Self::Result {
        for expr in &mut ast.borrow_mut().exprs {
            self.visit_expr(expr)?;
        }

        None
    }

    /// Type-check expr:
    /// - dispatch type-checking according to the kind of expr
    fn visit_expr(&mut self, expr: &mut Expr) -> Self::Result {
        match expr {
            Expr::PrintExpr(ref mut print_expr) => self.visit_print_expr(print_expr),
            Expr::VnameExpr(ref mut vname_expr) => self.visit_vname_expr(vname_expr),
            Expr::IntegerExpr(ref mut int_expr) => self.visit_integer_expr(int_expr),
            Expr::BoolExpr(ref mut bool_expr) => self.visit_bool_expr(bool_expr),
            Expr::UnaryExpr(ref mut un_expr) => self.visit_unary_expr(un_expr),
            Expr::AssignExpr(ref mut ass_expr) => self.visit_assign_expr(ass_expr),
            Expr::BinaryExpr(ref mut bin_expr) => self.visit_binary_expr(bin_expr),
        }
    }

    /// Type-check print expr:
    /// - type-check the expr
    fn visit_print_expr(&mut self, print_expr: &mut Expr) -> Self::Result {
        self.visit_expr(print_expr)
    }

    /// Type-check vname expr;
    /// - simply visit the identifier and return its type.
    fn visit_vname_expr(&mut self, vname_expr: &mut VnameExpr) -> Self::Result {
        vname_expr.typ = self.visit_identifier(&mut vname_expr.id);
        vname_expr.typ.clone()
    }

    /// Type-check integer expr:
    fn visit_integer_expr(&mut self, _int_expr: &mut i32) -> Self::Result {
        Some(Type::IntType)
    }

    /// Type-check bool expr:
    fn visit_bool_expr(&mut self, _bool_expr: &mut bool) -> Self::Result {
        Some(Type::BoolType)
    }

    /// Type-check an identifier:
    /// - for now, simply return its type, if any.
    fn visit_identifier(&mut self, id: &mut Identifier) -> Self::Result {
        if let Some(DeclOrId::Id(ref id_decl)) = self.id_table.get_attr(&id.spelling) {
            id.typ = id_decl.typ.clone();
            id.typ.clone()
        } else {
            None
        }
    }

    /// Type-check unary expr:
    /// - get the spec for the operator from the id table
    /// - get the type of the unary expr post typechecking
    /// - validate that the types are the same.
    fn visit_unary_expr(&mut self, un_expr: &mut UnaryExpr) -> Self::Result {
        let elem_typ = self.visit_expr(&mut un_expr.elem);
        if elem_typ.is_none() {
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!("for unary expr, elem type is unavailable"),
                ),
                None,
            );
        }

        let elem_typ = elem_typ.unwrap();
        let op_spec = self.get_unary_op_spec(&un_expr.op);

        if let DeclOrId::Decl(Decl::OperatorDecl(OperatorDecl::UnaryOperatorDecl(ref op_decl))) =
            op_spec
        {
            if op_decl.elem_typ != elem_typ {
                report_error(ExprError::new(ExprErrorKind::CheckerError, format!("unary expression elem type ({:?}) does not match the expected elem type ({:?}) for operator", 
                        elem_typ, op_decl.elem_typ)), None);
            }

            un_expr.typ = Some(op_decl.ret_typ.clone());
            un_expr.typ.clone()
        } else {
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!(
                        "expected a unary operator in unary expression, but found {:?}",
                        op_spec
                    ),
                ),
                None,
            );
            None
        }
    }

    /// Type-check assignment expr:
    /// - type-check vname and get id.
    /// - type-check rhs expr and get type.
    /// - get spec for assignment operation.
    /// - validate types
    /// - decorate identifier with type (if not set), and if already set,
    /// verify that the type has not changed.
    fn visit_assign_expr(&mut self, ass_expr: &mut AssignExpr) -> Self::Result {
        let lhs_typ = self.visit_expr(&mut ass_expr.vname);
        let rhs_typ = self.visit_expr(&mut ass_expr.expr);

        if rhs_typ.is_none() {
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!("could not determine type of rhs of assignment expression"),
                ),
                None,
            );
        }

        let op_spec = self.get_bin_op_spec(&ass_expr.op);
        if let DeclOrId::Decl(Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(ref _op_decl))) =
            op_spec
        {
            if let Expr::VnameExpr(ref mut vname) = *ass_expr.vname {
                if vname.typ.is_none() {
                    vname.typ = rhs_typ.clone();
                    vname.id.typ = rhs_typ.clone();
                    self.id_table
                        .save_attr(&vname.id.spelling, DeclOrId::Id(vname.id.clone()));
                } else if lhs_typ != rhs_typ {
                    report_error(
                        ExprError::new(
                            ExprErrorKind::CheckerError,
                            format!(
                                "id {:?} has inferred type {:?}, but rhs has type {:?}",
                                vname.id.spelling, vname.id.typ, rhs_typ
                            ),
                        ),
                        None,
                    );
                    return None;
                }
                ass_expr.typ = rhs_typ.clone();
                ass_expr.typ.clone()
            } else {
                report_error(
                    ExprError::new(
                        ExprErrorKind::CheckerError,
                        format!("the lhs of an assignment expression must be a vname"),
                    ),
                    None,
                );
                None
            }
        } else {
            None
        }
    }

    /// Type-check binary expr:
    /// - get the spec for the binary operator
    /// - type-check the lhs
    /// - type-check the rhs
    /// - validate types against spec
    /// - set the spec return type as the type of the expr
    fn visit_binary_expr(&mut self, bin_expr: &mut BinaryExpr) -> Self::Result {
        let lhs_typ = self.visit_expr(&mut bin_expr.lhs);
        if lhs_typ.is_none() {
            println!("{:?}", bin_expr.lhs);
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!("for bin expr, lhs type is unavailable"),
                ),
                None,
            );
        }

        let rhs_typ = self.visit_expr(&mut bin_expr.rhs);

        if rhs_typ.is_none() {
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!("for bin expr, rhs type is unavailable"),
                ),
                None,
            );
        }

        let lhs_typ = lhs_typ.unwrap();
        let rhs_typ = rhs_typ.unwrap();
        let op_spec = self.get_bin_op_spec(&bin_expr.op);

        if let DeclOrId::Decl(Decl::OperatorDecl(OperatorDecl::BinaryOperatorDecl(ref op_decl))) =
            op_spec
        {
            if op_decl.lhs_typ == Type::AnyType && op_decl.rhs_typ == Type::AnyType {
                if lhs_typ != rhs_typ {
                    report_error(ExprError::new(ExprErrorKind::CheckerError, format!("for bin expr, lhs type of lhs expr ({:?}) does not match the rhs expr type ({:?})", lhs_typ,
            rhs_typ)), None);
                }
            } else if lhs_typ != op_decl.lhs_typ {
                report_error(ExprError::new(ExprErrorKind::CheckerError, format!("for bin expr, lhs type of lhs expr ({:?}) does not match the spec's lhs type ({:?})", lhs_typ,
            op_decl.lhs_typ)), None);
            } else if rhs_typ != op_decl.rhs_typ {
                report_error(ExprError::new(ExprErrorKind::CheckerError, format!("for bin expr, lhs type of rhs expr ({:?}) does not match the spec's rhs type ({:?})", rhs_typ,
            op_decl.rhs_typ)), None);
            }

            bin_expr.typ = Some(op_decl.ret_typ.clone());
            bin_expr.typ.clone()
        } else {
            report_error(
                ExprError::new(
                    ExprErrorKind::CheckerError,
                    format!(
                        "expected a binary operator in binary expression, but found {:?}",
                        op_spec
                    ),
                ),
                None,
            );

            None
        }
    }
}
