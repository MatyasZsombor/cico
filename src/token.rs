#[derive(PartialEq, Debug)]
pub enum TokenType
{
    None,

    Ident,
    Number,
    Eof,

    Plus, //
    Minus, //
    Times, //
    Slash,
    Rem, //
    Eql, //
    Neq, //
    Lss, //
    Leq, //
    Gtr, //
    Geq, //
    And, //
    Or, //
    Assign, //
    Pplus, //
    Mminus, //
    Semicolon, //
    Comma, //
    Period, //
    Lpar, //
    Rpar, //
    Lbrack, //
    Rbrack, //
    Lbrace, //
    Rbrace, //

    Break,
    Struct,
    Else,
    Const,
    If,
    New,
    Return,
    Void,
    While
}

#[derive(PartialEq, Debug)]
pub struct Token
{
    pub token_type: TokenType,
    pub line: i32,
    pub col: i32,
    pub val: String,
}