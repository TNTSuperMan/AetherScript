pub struct Token {
    pub kind: TokenKind,
    pub at: usize,
}

pub enum TokenKind {
    Space,
    Identifier(String),
    Literal(Literal),
    WordSymbol(WordSymbol),
    Symbol(Symbol),
}

pub enum Literal {
    String(String),
    BigInt(String),
    Int(i64),
    Float(f64),
}

pub enum WordSymbol {
    Let,
    Mutable,
    Await,
    Async,
    Function,
    Return,
    Struct,
    Enum,
    Impl,

    Num,
    Smi,
    Str,
    Bool,
    Unique,
}

pub enum Symbol {
    Semi,
    Comma,
    Dot,
    OpenParen,
    CloseParen,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    At,
    Pound,
    Tilde,
    Question,
    Colon,
    Dollar,
    Eq,
    Bang,
    Lt,
    Gt,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Or,
    And,
    Caret,
}
