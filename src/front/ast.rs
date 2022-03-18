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
    AssignExpr(AssignExpr),
    BinaryExpr(BinaryExpr),
    BoolExpr(bool),
    IntegerExpr(i32),
    PrintExpr(Box<Expr>),
    UnaryExpr(UnaryExpr),
    VnameExpr(VnameExpr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct VnameExpr {
    pub id: Identifier,
    pub typ: Option<Type>,
}

impl VnameExpr {
    pub fn new(id: Identifier) -> Self {
        VnameExpr { id, typ: None }
    }
}

impl VnameExpr {}

#[derive(Debug)]
pub struct AssignExpr {
    pub vname: Box<Expr>, // VnameExpr
    pub op: BinaryOperator,
    pub expr: Box<Expr>,
    pub typ: Option<Type>,
}

impl AssignExpr {
    pub fn new(vname: Box<Expr>, op: BinaryOperator, expr: Box<Expr>) -> Self {
        AssignExpr {
            vname,
            op,
            expr,
            typ: None,
        }
    }
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub op: UnaryOperator,
    pub elem: Box<Expr>,
    pub typ: Option<Type>,
}

impl UnaryExpr {
    pub fn new(op: UnaryOperator, elem: Box<Expr>) -> Self {
        UnaryExpr {
            op,
            elem,
            typ: None,
        }
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub lhs: Box<Expr>,
    pub op: BinaryOperator,
    pub rhs: Box<Expr>,
    pub typ: Option<Type>,
}

impl BinaryExpr {
    pub fn new(lhs: Box<Expr>, op: BinaryOperator, rhs: Box<Expr>) -> Self {
        BinaryExpr {
            lhs,
            op,
            rhs,
            typ: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Identifier {
    pub spelling: String,
    pub typ: Option<Type>,
}

impl Identifier {
    pub fn new(spelling: String) -> Self {
        Identifier {
            spelling,
            typ: None,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOperator {
    BitwiseNot,
    LogicalNot,
    UnaryMinus,
    UnaryPlus,
}

#[derive(Debug, Copy, Clone)]
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

// for the std env

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    AnyType,
    BoolType,
    IntType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Decl {
    ConstDecl(ConstDecl),
    OperatorDecl(OperatorDecl),
    IdDecl(),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConstDecl {
    IntegerLiteral(i32),
    BoolLiteral(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorDecl {
    UnaryOperatorDecl(UnaryOperatorDecl),
    BinaryOperatorDecl(BinaryOperatorDecl),
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryOperatorDecl {
    pub elem_typ: Type,
    pub ret_typ: Type,
}

impl UnaryOperatorDecl {
    pub fn new(elem_typ: Type, ret_typ: Type) -> Self {
        UnaryOperatorDecl { elem_typ, ret_typ }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperatorDecl {
    pub lhs_typ: Type,
    pub rhs_typ: Type,
    pub ret_typ: Type,
}

impl BinaryOperatorDecl {
    pub fn new(lhs_typ: Type, rhs_typ: Type, ret_typ: Type) -> Self {
        BinaryOperatorDecl {
            lhs_typ,
            rhs_typ,
            ret_typ,
        }
    }
}
