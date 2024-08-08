use std::str::Chars;
use crate::token::{Token, TokenType};
use crate::token::TokenType::{And, Assign, Break, Comma, Const, Else, Eof, Eql, Geq, Gtr, Ident, If, Lbrace, Lbrack, Leq, Lpar, Lss, Minus, Mminus, Neq, New, Number, Or, Period, Plus, Pplus, Rbrace, Rbrack, Rem, Return, Rpar, Semicolon, Slash, Struct, Times, Void, While};

pub struct Lexer<'a> {
    input: Chars<'a>,
    line: i32,
    col: i32,
    ch: u8,
}

impl Lexer<'_> {
    pub fn init(input: Chars) -> Lexer
    {
        let mut lexer = Lexer { input, line: 1, col: 0, ch: 0 };
        lexer.next_ch();
        lexer
    }

    pub fn next(&mut self) -> Token
    {
        while self.ch <= b' ' && self.ch != 0 {
            self.next_ch();
        }

        let mut token = Token { token_type: TokenType::None, line: self.line, col: self.col, val: String::from("") };
        token.token_type = match self.ch.to_ascii_lowercase() {
            b'a'..=b'z' | b'A'..=b'Z' => self.read_name(&mut token),
            b'0'..=b'9' => self.read_number(&mut token),

            b'*' => {
                self.next_ch();
                Times
            }
            b'%' => {
                self.next_ch();
                Rem
            }
            b',' => {
                self.next_ch();
                Comma
            }
            b';' => {
                self.next_ch();
                Semicolon
            }
            b'.' => {
                self.next_ch();
                Period
            }
            b'(' => {
                self.next_ch();
                Lpar
            }
            b')' => {
                self.next_ch();
                Rpar
            }
            b'[' => {
                self.next_ch();
                Lbrack
            }
            b']' => {
                self.next_ch();
                Rbrack
            }
            b'{' => {
                self.next_ch();
                Lbrace
            }
            b'}' => {
                self.next_ch();
                Rbrace
            }

            b'+' => {
                self.next_ch();
                if self.ch == b'+'
                {
                    self.next_ch();
                    Pplus
                } else { Plus }
            }
            b'-' => {
                self.next_ch();
                if self.ch == b'-'
                {
                    self.next_ch();
                    Mminus
                } else { Minus }
            }
            b'=' => {
                self.next_ch();
                if self.ch == b'='
                {
                    self.next_ch();
                    Eql
                } else { Assign }
            }
            b'!' => {
                self.next_ch();
                if self.ch == b'=' {
                    self.next_ch();
                    Neq
                } else { TokenType::None }
            }
            b'<' => {
                self.next_ch();
                if self.ch == b'=' {
                    self.next_ch();
                    Leq
                } else { Lss }
            }
            b'>' => {
                self.next_ch();
                if self.ch == b'=' {
                    self.next_ch();
                    Geq
                } else { Gtr }
            }
            b'&' => {
                self.next_ch();
                if self.ch == b'&' {
                    self.next_ch();
                    And
                } else { TokenType::None }
            }
            b'|' => {
                self.next_ch();
                if self.ch == b'|' {
                    self.next_ch();
                    Or
                } else { TokenType::None }
            }

            b'/' => {
                self.next_ch();
                if self.ch == b'/' {
                    self.next_ch();
                    while self.ch != b'\n' && self.ch != 0 {
                        self.next_ch();
                    }
                    token = self.next();
                    token.token_type
                } else { Slash }
            }

            0 => Eof,

            _ => { TokenType::None }
        };

        token
    }

    fn next_ch(&mut self) {
        let next = self.input.next();

        if next.is_some()
        {
            self.ch = next.unwrap() as u8;
            self.col += 1;

            if self.ch == b'\n'
            {
                self.line += 1;
                self.col = 0;
            }
        } else { self.ch = 0; }
    }

    fn read_name(&mut self, token: &mut Token) -> TokenType {
        let mut value = vec![self.ch as char];
        self.next_ch();

        while (self.ch >= b'a' && self.ch <= b'z') || (self.ch >= b'0' && self.ch <= b'9') || (self.ch >= b'A' && self.ch <= b'Z') {
            value.push(self.ch as char);
            self.next_ch();
        }
        token.val = value.into_iter().collect();
        Self::look_up_ident(&token.val)
    }

    fn look_up_ident(value: &str) -> TokenType
    {
        match value {
            "break" => Break,
            "struct" => Struct,
            "else" => Else,
            "const" => Const,
            "if" => If,
            "new" => New,
            "return" => Return,
            "void" => Void,
            "while" => While,
            _ => Ident
        }
    }

    fn read_number(&mut self, token: &mut Token) -> TokenType {
        let mut value = vec![self.ch as char];
        self.next_ch();

        while self.ch >= b'0' && self.ch <= b'9' {
            value.push(self.ch as char);
            self.next_ch();
        }
        token.val = value.into_iter().collect();
        Number
    }
}

#[cfg(test)]
mod lexer_test {
    use super::*;

    #[test]
    fn test_operators()
    {
        let test = "+-*/%==!=<<=>>=&&||=++--;,.()[]{}";
        let expected_types = vec![Plus, Minus, Times, Slash,
                                  Rem, Eql, Neq, Lss,
                                  Leq, Gtr, Geq, And,
                                  Or, Assign, Pplus,
                                  Mminus, Semicolon,
                                  Comma, Period, Lpar,
                                  Rpar, Lbrack, Rbrack,
                                  Lbrace, Rbrace, Eof];

        let mut lexer = Lexer::init(test.chars());

        for expected_type in expected_types {
            assert_eq!(expected_type, lexer.next().token_type);
        }
    }

    #[test]
    fn test_identifiers()
    {
        let test = "break\nstruct\nelse\nconst\nif\nnew\nreturn\nvoid\nwhile\nx\nabc\n100";
        let expected_types = vec![Break, Struct, Else,
                                  Const, If, New, Return, Void, While, Ident, Ident, Number, Eof];
        let expected_values = vec!["break", "struct", "else", "const",
                                   "if", "new", "return", "void",
                                   "while", "x", "abc", "100", ""];

        let mut lexer = Lexer::init(test.chars());

        for i in 0..expected_types.len() {
            let token = lexer.next();
            assert_eq!(expected_types[i], token.token_type);
            assert_eq!(expected_values[i], token.val)
        }
    }

    #[test]
    fn test_comment()
    {
        let test = "//This is a comment";
        let mut lexer = Lexer::init(test.chars());

        assert_eq!(Eof, lexer.next().token_type)
    }

    #[test]
    fn test_comment_with_code()
    {
        let test = "//This is a comment\nint x = 10;";
        let mut lexer = Lexer::init(test.chars());

        let expected_types = vec![Ident, Ident, Assign, Number, Semicolon, Eof];
        let expected_values = vec!["int", "x", "", "10", "", ""];

        for i in 0..expected_types.len() {
            let token = lexer.next();
            assert_eq!(expected_types[i], token.token_type);
            assert_eq!(expected_values[i], token.val)
        }
    }
}