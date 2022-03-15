#[derive(Debug)]
pub enum TokenKind {
    BitwiseAnd,
    BitwiseNot,
    BitwiseOr,
    BitwiseXor,
    Eof,
    LeftParen,
    LeftShift,
    LeftShitAssign,
    LogicalAnd,
    LogicalNot,
    LogicalOr,
    Minus,
    MinusAssign,
    Mod,
    ModAssign,
    Plus,
    PlusAssign,
    RightParen,
    RightShift,
    RightShiftAssign,
    Slash,
    SlashAssign,
    Star,
    StarAssign,
}

#[derive(Debug)]
pub struct Token {}
