use super::ast::*;
use super::token::{Token, TokenKind};
use crate::error::{report_error, ExprError, ExprErrorKind};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Parser {
    tokens: Vec<Token>,
    curr_idx: usize,
}

impl Parser {
    const MIN_BINDING_POWER: i32 = -1;
    const MAX_BINDING_POWER: i32 = 120;

    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            curr_idx: 0,
        }
    }

    fn curr_token(&self) -> &Token {
        &self.tokens[self.curr_idx]
    }

    fn advance_if(&mut self, expected_kind: TokenKind) {
        if self.curr_token().kind != expected_kind {
            report_error(
                ExprError::new(
                    ExprErrorKind::ParserError,
                    format!(
                        "expected token {:?}, but found token {:?}",
                        expected_kind,
                        self.curr_token().kind
                    ),
                ),
                Some(&self.curr_token().loc),
            );
        } else {
            self.curr_idx += 1;
        }
    }

    fn advance(&mut self) {
        self.curr_idx += 1;
    }
    fn lbp(kind: TokenKind) -> i32 {
        match kind {
            TokenKind::Assign
            | TokenKind::PlusAssign
            | TokenKind::MinusAssign
            | TokenKind::StarAssign
            | TokenKind::SlashAssign
            | TokenKind::ModAssign
            | TokenKind::LeftShiftAssign
            | TokenKind::RightShiftAssign
            | TokenKind::BitwiseOrAssign
            | TokenKind::BitwiseXorAssign => 10,
            TokenKind::LogicalOr => 20,
            TokenKind::LogicalAnd => 30,
            TokenKind::BitwiseOr => 40,
            TokenKind::BitwiseXor => 50,
            TokenKind::Equal | TokenKind::NotEqual => 70,
            TokenKind::LessThan
            | TokenKind::LessThanOrEqual
            | TokenKind::GreaterThan
            | TokenKind::GreaterThanOrEqual => 80,
            TokenKind::LeftShift | TokenKind::RightShift => 90,
            TokenKind::Plus | TokenKind::Minus => 100,
            TokenKind::Star | TokenKind::Slash | TokenKind::Mod => 110,
            TokenKind::LogicalNot | TokenKind::BitwiseNot => 120,
            _ => Parser::MIN_BINDING_POWER,
        }
    }

    /// Check if the given operator is right-associative or not.
    fn is_right_associative(kind: TokenKind) -> bool {
        match kind {
            TokenKind::LogicalNot
            | TokenKind::BitwiseNot
            | TokenKind::Star
            | TokenKind::Assign
            | TokenKind::PlusAssign
            | TokenKind::MinusAssign
            | TokenKind::StarAssign
            | TokenKind::SlashAssign
            | TokenKind::ModAssign
            | TokenKind::BitwiseOrAssign
            | TokenKind::BitwiseXorAssign => true,
            _ => false,
        }
    }

    /// The left denotations - handles binary expressions.
    fn led(&mut self, lhs: Expr, kind: TokenKind, rhs: Expr) -> Expr {
        let op = match kind {
            TokenKind::Assign => BinaryOperator::Assign,
            TokenKind::BitwiseAnd => BinaryOperator::BitwiseAnd,
            TokenKind::BitwiseAndAssign => BinaryOperator::BitwiseAndAssign,
            TokenKind::BitwiseOr => BinaryOperator::BitwiseOr,
            TokenKind::BitwiseOrAssign => BinaryOperator::BitwiseOrAssign,
            TokenKind::BitwiseXor => BinaryOperator::BitwiseXor,
            TokenKind::BitwiseXorAssign => BinaryOperator::BitwiseXorAssign,
            TokenKind::Equal => BinaryOperator::Equal,
            TokenKind::GreaterThan => BinaryOperator::GreaterThan,
            TokenKind::GreaterThanOrEqual => BinaryOperator::GreaterThanOrEqual,
            TokenKind::LeftShift => BinaryOperator::LeftShift,
            TokenKind::LeftShiftAssign => BinaryOperator::LeftShiftAssign,
            TokenKind::LessThan => BinaryOperator::LessThan,
            TokenKind::LessThanOrEqual => BinaryOperator::LessThanOrEqual,
            TokenKind::LogicalAnd => BinaryOperator::LogicalAnd,
            TokenKind::LogicalOr => BinaryOperator::LogicalOr,
            TokenKind::Minus => BinaryOperator::Sub,
            TokenKind::MinusAssign => BinaryOperator::SubAssign,
            TokenKind::Mod => BinaryOperator::Mod,
            TokenKind::ModAssign => BinaryOperator::ModAssign,
            TokenKind::NotEqual => BinaryOperator::NotEqual,
            TokenKind::Plus => BinaryOperator::Add,
            TokenKind::PlusAssign => BinaryOperator::AddAssign,
            TokenKind::RightShift => BinaryOperator::RightShift,
            TokenKind::RightShiftAssign => BinaryOperator::RightShiftAssign,
            TokenKind::Slash => BinaryOperator::Div,
            TokenKind::SlashAssign => BinaryOperator::DivAssign,
            TokenKind::Star => BinaryOperator::Mul,
            TokenKind::StarAssign => BinaryOperator::MulAssign,
            _ => unreachable!(),
        };

        Expr::BinaryExpr {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    /// The null denotation - handles unary expressions.
    fn nud(&mut self, token: Token) -> Expr {
        match token.kind {
            TokenKind::LeftParen => {
                let expr = self.parse_expression(Parser::MIN_BINDING_POWER);
                if self.curr_token().kind != TokenKind::RightParen {
                    report_error(
                        ExprError::new(
                            ExprErrorKind::ParserError,
                            format!("Missing right parenthesis while parsing expression"),
                        ),
                        Some(&token.loc),
                    );
                }
                self.advance();
                expr
            }

            TokenKind::Plus => Expr::UnaryExpr {
                op: UnaryOperator::UnaryPlus,
                elem: Box::new(self.parse_expression(Parser::MAX_BINDING_POWER)),
            },

            TokenKind::Minus => Expr::UnaryExpr {
                op: UnaryOperator::UnaryMinus,
                elem: Box::new(self.parse_expression(Parser::MIN_BINDING_POWER)),
            },

            TokenKind::Print => {
                Expr::PrintExpr(Box::new(self.parse_expression(Parser::MIN_BINDING_POWER)))
            }

            TokenKind::Integer => Expr::IntegerExpr(token.spelling.parse::<i32>().unwrap()),

            TokenKind::False | TokenKind::True => {
                Expr::BoolExpr(token.spelling.parse::<bool>().unwrap())
            }

            TokenKind::Identifier => Expr::VnameExpr(token.spelling.clone()),

            _ => {
                eprintln!(
                    "In unreachable inside nud, token = {:?}, curr_token = {:?}",
                    token,
                    self.curr_token()
                );
                unreachable!()
            }
        }
    }

    fn parse_expression(&mut self, rbp: i32) -> Expr {
        let token = self.curr_token().clone();
        self.advance();
        let mut left = self.nud(token);

        while rbp < Parser::lbp(self.curr_token().kind) {
            let token = self.curr_token().clone();
            self.advance();
            let right = if Parser::is_right_associative(token.kind) {
                self.parse_expression(Parser::lbp(token.kind) - 1)
            } else {
                self.parse_expression(Parser::lbp(token.kind))
            };

            left = self.led(left, token.kind, right);
        }

        left
    }

    /// Ast ::= Expr* Eof
    pub fn parse(&mut self) -> SharedPtr<Ast> {
        let mut exprs = Vec::new();

        while self.curr_token().kind != TokenKind::Eof {
            exprs.push(self.parse_expression(Parser::MIN_BINDING_POWER));
        }

        Rc::new(RefCell::new(Ast::new(exprs)))
    }
}
