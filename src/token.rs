pub enum TokenType
{
    None,

    Ident,
    Number,
    Char,

    Plus,
    Minus,
    Times,
    Slash,
    Rem,
    Eql,
    Neq,
    Lss,
    Leq,
    Gtr,
    Geq,
    And,
    Or,
    Assign,
    Pplus,
    Mminus,
    Comma,
    Period,
    Lpar,
    Rpar,
    Lbrack,
    Rbrack,
    Lbrace,
    Rbrace,

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

pub struct Token<'a>
{
    pub token_type: TokenType,
    pub line: i32,
    pub col: i32,
    pub val: &'a str,
    pub num_val: i32
}