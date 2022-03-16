use super::source_file::Location;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenKind {
    Assign,
    BitwiseAnd,
    BitwiseAndAssign,
    BitwiseNot,
    BitwiseOr,
    BitwiseOrAssign,
    BitwiseXor,
    BitwiseXorAssign,
    Eof,
    Equal,
    False,
    GreaterThan,
    GreaterThanOrEqual,
    Identifier,
    Integer,
    LeftParen,
    LeftShift,
    LeftShiftAssign,
    LessThan,
    LessThanOrEqual,
    LogicalAnd,
    LogicalNot,
    LogicalOr,
    Minus,
    MinusAssign,
    Mod,
    ModAssign,
    NotEqual,
    Plus,
    PlusAssign,
    Print,
    RightParen,
    RightShift,
    RightShiftAssign,
    Slash,
    SlashAssign,
    Star,
    StarAssign,
    True,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub spelling: String,
    pub loc: Location,
}

impl Token {
    pub fn new(kind: TokenKind, spelling: String, loc: Location) -> Self {
        let kind = match &*spelling {
            "print" => TokenKind::Print,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            _ => kind,
        };

        Token {
            kind,
            spelling,
            loc,
        }
    }
}
