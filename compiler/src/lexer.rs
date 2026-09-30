use crate::lexer::parser::LexerParser;

mod chars_while;
mod interner;
mod parser;

pub fn parse_to_lexer(code: &str) -> Vec<Token> {
    let mut parser = LexerParser::new(code);
    parser.parse();
    parser.take_toks()
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Space,
    Identifier(String),
    Literal(Literal),
    WordSymbol(WordSymbol),
    Symbol(Symbol),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    BigInt(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug, Clone)]
pub enum WordSymbol {
    Let,
    Mut,
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

#[derive(Debug, Clone)]
pub enum Symbol {
    Semi,         // ;
    Comma,        // ,
    Dot,          // .
    OpenParen,    // (
    CloseParen,   // )
    OpenBrace,    // {
    CloseBrace,   // }
    OpenBracket,  // [
    CloseBracket, // ]
    At,           // @
    Pound,        // #
    Tilde,        // ~
    Question,     // ?
    Colon,        // :
    Dollar,       // $
    Eq,           // =
    Bang,         // !
    Lt,           // <
    Gt,           // >
    Plus,         // +
    Minus,        // -
    Star,         // *
    Slash,        // /
    Percent,      // %
    Or,           // |
    And,          // &
    Caret,        // ^
}
