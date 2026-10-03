use crate::lexer::{
    error::LexerError,
    interner::{IdentifierId, IdentifierInterner},
    parser::LexerParser,
};

mod chars_while;
mod error;
pub mod interner;
mod num;
mod parser;
mod str;

pub fn parse_to_lexer(
    code: &str,
    interner: &mut IdentifierInterner,
) -> Result<Vec<Token>, LexerError> {
    let mut parser = LexerParser::new(code, interner);
    parser.parse()?;
    Ok(parser.take_toks())
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub at: usize,
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    // Space,
    Identifier(IdentifierId),
    Literal(Literal),
    WordSymbol(WordSymbol),
    Symbol(Symbol),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Smi(i32),
    Float(f64),
    BigInt(String),
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
