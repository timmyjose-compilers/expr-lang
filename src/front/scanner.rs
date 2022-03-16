use super::source_file::{Char, Location, NUL};
use super::token::{Token, TokenKind};
use crate::error::{report_error, ExprError, ExprErrorKind};

pub struct Scanner {
    chars: Vec<Char>,
    pub tokens: Vec<Token>,
    curr_idx: usize,
    curr_buf: String,
    curr_loc: Location,
}

impl Scanner {
    pub fn new(chars: Vec<Char>) -> Self {
        Scanner {
            chars,
            tokens: Vec::new(),
            curr_idx: 0,
            curr_buf: String::default(),
            curr_loc: Location::default(),
        }
    }

    fn check_character_stream(&self) {
        if self.curr_idx >= self.chars.len() {
            report_error(
                ExprError::new(
                    ExprErrorKind::ScannerError,
                    format!("ran out of characters"),
                ),
                None,
            );
        }
    }

    fn curr_char(&self) -> &Char {
        self.check_character_stream();
        &self.chars[self.curr_idx]
    }

    fn peek_char(&self, offset: usize) -> Option<&Char> {
        if self.curr_idx + offset >= self.chars.len() {
            None
        } else {
            Some(&self.chars[self.curr_idx + offset])
        }
    }

    fn skip_it(&mut self) {
        self.curr_idx += 1;
    }

    fn eat_it(&mut self) {
        self.curr_buf.push(self.curr_char().c);
        self.curr_idx += 1;
    }

    fn skip_whitespace(&mut self) {
        match self.curr_char().c {
            '/' => {
                if let Some(ch) = self.peek_char(1) {
                    if ch.c == '/' {
                        self.skip_it();
                        self.skip_it();
                        while self.curr_char().c != NUL && self.curr_char().c != '\n' {
                            self.skip_it();
                        }

                        if self.curr_char().c == '\n' {
                            self.skip_it();
                        }
                    }
                }
            }

            c if c.is_whitespace() => {
                while self.curr_char().c.is_whitespace() {
                    self.skip_it();
                }
            }
            _ => {}
        }
    }

    fn scan_token(&mut self) -> TokenKind {
        self.curr_loc = self.curr_char().loc.clone();

        match self.curr_char().c {
            '(' => {
                self.eat_it();
                TokenKind::LeftParen
            }

            ')' => {
                self.eat_it();
                TokenKind::RightParen
            }

            '+' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }

            '-' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::MinusAssign
                } else {
                    TokenKind::Minus
                }
            }

            '*' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }

            '/' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }

            '%' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::ModAssign
                } else {
                    TokenKind::Mod
                }
            }

            '=' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    TokenKind::Equal
                } else {
                    TokenKind::Assign
                }
            }

            '!' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::NotEqual
                } else {
                    TokenKind::LogicalNot
                }
            }

            '&' => {
                self.eat_it();
                if self.curr_char().c == '&' {
                    self.eat_it();
                    TokenKind::LogicalAnd
                } else {
                    TokenKind::BitwiseAnd
                }
            }

            '|' => {
                self.eat_it();
                if self.curr_char().c == '|' {
                    self.eat_it();
                    TokenKind::LogicalOr
                } else {
                    TokenKind::BitwiseOr
                }
            }

            '^' => {
                self.eat_it();
                if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::BitwiseXorAssign
                } else {
                    TokenKind::BitwiseXor
                }
            }

            '~' => {
                self.eat_it();
                TokenKind::BitwiseNot
            }

            '<' => {
                self.eat_it();
                if self.curr_char().c == '<' {
                    self.eat_it();
                    if self.curr_char().c == '=' {
                        self.eat_it();
                        TokenKind::LeftShiftAssign
                    } else {
                        TokenKind::LeftShift
                    }
                } else if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::LessThanOrEqual
                } else {
                    TokenKind::LessThan
                }
            }

            '>' => {
                self.eat_it();
                if self.curr_char().c == '>' {
                    self.eat_it();
                    if self.curr_char().c == '=' {
                        self.eat_it();
                        TokenKind::RightShiftAssign
                    } else {
                        TokenKind::RightShift
                    }
                } else if self.curr_char().c == '=' {
                    self.eat_it();
                    TokenKind::GreaterThanOrEqual
                } else {
                    TokenKind::GreaterThan
                }
            }

            c if c.is_ascii_alphabetic() => {
                while self.curr_char().c.is_ascii_alphabetic() {
                    self.eat_it();
                }
                TokenKind::Identifier
            }

            c if c.is_ascii_digit() => {
                while self.curr_char().c.is_ascii_digit() {
                    self.eat_it();
                }
                TokenKind::Integer
            }

            NUL => TokenKind::Eof,

            _ => {
                println!(
                    "unreachable - {}, {}",
                    self.curr_char().c,
                    self.curr_char().c as i32
                );
                unreachable!();
            }
        }
    }

    fn scan(&mut self) -> Token {
        while self.curr_char().c.is_whitespace()
            || self.curr_char().c == '/'
                && self.peek_char(1).is_some()
                && self.peek_char(1).unwrap().c == '/'
        {
            self.skip_whitespace();
        }

        self.curr_buf = String::new();
        let kind = self.scan_token();
        Token::new(kind, self.curr_buf.clone(), self.curr_loc.clone())
    }

    pub fn scan_all(&mut self) {
        loop {
            let token = self.scan();
            let is_eof = token.kind == TokenKind::Eof;
            self.tokens.push(token);

            if is_eof {
                break;
            }
        }
    }
}
