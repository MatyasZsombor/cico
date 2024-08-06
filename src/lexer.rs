use std::str::Chars;
use crate::token::{Token, TokenType};
use crate::token::TokenType::{And, Assign, Comma, Eql, Geq, Gtr, Lbrace, Lbrack, Leq, Lpar, Lss, Minus, Mminus, Neq, Or, Period, Plus, Pplus, Rbrace, Rbrack, Rem, Rpar, Slash, Times};

struct Lexer<'a> {
    input: Chars<'a>,
    line: i32,
    col: i32,
    ch: char,
    eof: char,
}

impl Lexer<'_> {
    pub fn init(input: Chars) -> Lexer
    {
        Lexer { input, line: 1, col: 0, ch: '\0', eof: char::from_digit(3, 10).unwrap() }
    }

    pub fn next(&mut self) -> Token
    {
        while self.ch <= ' ' && self.ch != self.eof {
            self.next_ch();
        }

        let mut token = Token { token_type: TokenType::None, line: self.line, col: self.col, val: String::from("") };
        token.token_type = match self.ch.to_ascii_lowercase() {
            'a'..='z' | 'A'..='Z' => self.read_name(&mut token),
            '0'..='9' => self.read_number(&token),
            '\'' => self.read_char(&token),

            '*' => Times,
            '%' => Rem,
            ',' => Comma,
            '.' => Period,
            '(' => Lpar,
            ')' => Rpar,
            '[' => Lbrack,
            ']' => Rbrack,
            '{' => Lbrace,
            '}' => Rbrace,

            '+' => {
                self.next_ch();
                if self.ch == '+'
                {
                    Pplus
                } else { Plus }
            }
            '-' => {
                self.next_ch();
                if self.ch == '-'
                {
                    Mminus
                } else { Minus }
            }
            '=' => {
                self.next_ch();
                if self.ch == '='
                {
                    Eql
                } else { Assign }
            }
            '!' => {
                self.next_ch();
                if self.ch == '=' {
                    Neq
                } else { TokenType::None }
            }
            '<' => {
                self.next_ch();
                if self.ch == '=' {
                    Leq
                } else { Lss }
            }
            '>' => {
                self.next_ch();
                if self.ch == '=' {
                    Geq
                } else { Gtr }
            }
            '&' => {
                self.next_ch();
                if self.ch == '&' {
                    And
                } else { TokenType::None }
            }
            '|' => {
                self.next_ch();
                if self.ch == '|' {
                    Or
                } else { TokenType::None }
            }

            '/' => {
                self.next_ch();
                if self.ch == '/' {
                    self.next_ch();
                    let mut t = TokenType::None;

                    while self.ch != '\n' && self.ch != self.eof {
                        let tmp = self.next();
                        token.val = tmp.val;
                        token.col = tmp.col;
                        token.line = tmp.line;

                        t = tmp.token_type
                    }
                    t
                } else { Slash }
            }

            _ => { TokenType::None }
        };

        token
    }

    fn next_ch(&mut self) {
        let next = self.input.next();

        if next.is_some()
        {
            self.ch = next.unwrap();
            self.col += 1;

            if self.ch == '\n'
            {
                self.line += 1;
                self.col = 0;
            }
        }
    }

    fn read_name(&mut self, token: &mut Token) -> TokenType {
        self.next_ch();
        let kind = TokenType::None;
        let mut value = vec![];

        while (self.ch >= 'a' && self.ch <= 'z') || (self.ch >= '0' && self.ch <= '9') || (self.ch >= 'A' && self.ch <= 'Z')  {
            value.push(self.ch);
            self.next_ch();
        }
        token.val = value.into_iter().collect();
        kind
    }
}