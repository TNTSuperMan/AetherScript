use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::lexer::{
    Symbol, Token, TokenKind, WordSymbol, chars_while::CharsWhile, interner::LiteralInterner,
};

fn is_identifier_char<const IS_FIRST: bool>(c: char) -> bool {
    (!IS_FIRST && c.is_ascii_digit()) || c.is_ascii_alphabetic() || c == '_'
}

pub struct LexerParser<'a> {
    iter: Peekable<Enumerate<Chars<'a>>>,
    literals: LiteralInterner,
    toks: Vec<Token>,
}

impl<'a> LexerParser<'a> {
    pub fn new(code: &'a str) -> Self {
        LexerParser {
            iter: code.chars().enumerate().peekable(),
            literals: LiteralInterner::new(),
            toks: vec![],
        }
    }
    fn step(&mut self) {
        while let Some((at, c)) = self.iter.next() {
            let kind = match c {
                c if c.is_whitespace() => continue,
                c if is_identifier_char::<true>(c) => {
                    let identifier: String =
                        CharsWhile::new(&mut self.iter, is_identifier_char::<false>).collect();
                    match identifier.as_str() {
                        "let" => TokenKind::WordSymbol(WordSymbol::Let),
                        "mut" => TokenKind::WordSymbol(WordSymbol::Mut),
                        "await" => TokenKind::WordSymbol(WordSymbol::Await),
                        "async" => TokenKind::WordSymbol(WordSymbol::Async),
                        "fn" => TokenKind::WordSymbol(WordSymbol::Function),
                        "return" => TokenKind::WordSymbol(WordSymbol::Return),
                        "struct" => TokenKind::WordSymbol(WordSymbol::Struct),
                        "enum" => TokenKind::WordSymbol(WordSymbol::Enum),

                        "num" => TokenKind::WordSymbol(WordSymbol::Num),
                        "smi" => TokenKind::WordSymbol(WordSymbol::Smi),
                        "str" => TokenKind::WordSymbol(WordSymbol::Str),
                        "bool" => TokenKind::WordSymbol(WordSymbol::Bool),
                        "unique" => TokenKind::WordSymbol(WordSymbol::Unique),

                        _ => TokenKind::Identifier(identifier),
                    }
                }
                ';' => TokenKind::Symbol(Symbol::Semi),
                _ => todo!(),
            };
            self.toks.push(Token { kind, at });
        }
    }
}
