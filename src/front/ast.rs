use std::cell::RefCell;
use std::rc::Rc;

pub type SharedPtr<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct Ast {
    pub exprs: Vec<Expr>,
}

impl Ast {
    pub fn new(exprs: Vec<Expr>) -> Self {
        Ast { exprs }
    }
}

#[derive(Debug)]
pub enum Expr {
    UnaryExpr {
        op: UnaryOperator,
        elem: Box<Expr>,
    },
    BinaryExpr {
        lhs: Box<Expr>,
        op: BinaryOperator,
        rhs: Box<Expr>,
    },
    PrintExpr(Box<Expr>),
    VnameExpr(String),
    IntegerExpr(i32),
    BoolExpr(bool),
}

#[derive(Debug)]
pub enum UnaryOperator {
    BitwiseNot,
    LogicalNot,
    UnaryMinus,
    UnaryPlus,
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    AddAssign,
    Assign,
    BitwiseAnd,
    BitwiseAndAssign,
    BitwiseOr,
    BitwiseOrAssign,
    BitwiseXor,
    BitwiseXorAssign,
    Div,
    DivAssign,
    Equal,
    GreaterThan,
    GreaterThanOrEqual,
    LeftShift,
    LeftShiftAssign,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalAndAssign,
    LogicalOr,
    LogicalOrAssign,
    Mod,
    ModAssign,
    Mul,
    MulAssign,
    NotEqual,
    RightShift,
    RightShiftAssign,
    Sub,
    SubAssign,
}
