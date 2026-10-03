use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use crate::lexer::{
    Literal, Symbol, Token, TokenKind, WordSymbol,
    chars_while::CharsWhile,
    error::LexerError,
    interner::IdentifierInterner,
    num::{TryToNumstrResult, try_iter_to_num},
    str::parse_string,
};

fn is_identifier_char<const IS_FIRST: bool>(c: char) -> bool {
    (!IS_FIRST && c.is_ascii_digit()) || c.is_ascii_alphabetic() || c == '_'
}

fn try_into_word_symbol(s: &str) -> Option<WordSymbol> {
    Some(match s {
        "let" => WordSymbol::Let,
        "mut" => WordSymbol::Mut,
        "await" => WordSymbol::Await,
        "async" => WordSymbol::Async,
        "fn" => WordSymbol::Function,
        "return" => WordSymbol::Return,
        "struct" => WordSymbol::Struct,
        "enum" => WordSymbol::Enum,

        "smi" => WordSymbol::Smi,
        "float" => WordSymbol::Float,
        "bigint" => WordSymbol::Bigint,
        "str" => WordSymbol::Str,
        "bool" => WordSymbol::Bool,
        "unique" => WordSymbol::Unique,
        _ => return None,
    })
}

fn try_into_symbol(c: char) -> Option<Symbol> {
    Some(match c {
        ';' => Symbol::Semi,
        ',' => Symbol::Comma,
        '.' => Symbol::Dot,
        '(' => Symbol::OpenParen,
        ')' => Symbol::CloseParen,
        '{' => Symbol::OpenBrace,
        '}' => Symbol::CloseBrace,
        '[' => Symbol::OpenBracket,
        ']' => Symbol::CloseBracket,
        '@' => Symbol::At,
        '#' => Symbol::Pound,
        '~' => Symbol::Tilde,
        '?' => Symbol::Question,
        ':' => Symbol::Colon,
        '$' => Symbol::Dollar,
        '=' => Symbol::Eq,
        '!' => Symbol::Bang,
        '<' => Symbol::Lt,
        '>' => Symbol::Gt,
        '+' => Symbol::Plus,
        '-' => Symbol::Minus,
        '*' => Symbol::Star,
        '/' => Symbol::Slash,
        '%' => Symbol::Percent,
        '|' => Symbol::Or,
        '&' => Symbol::And,
        '^' => Symbol::Caret,
        _ => return None,
    })
}

pub struct LexerParser<'a> {
    iter: Peekable<Enumerate<Chars<'a>>>,
    ids: &'a mut IdentifierInterner,
    toks: Vec<Token>,
}

impl<'a> LexerParser<'a> {
    pub fn new(code: &'a str, interner: &'a mut IdentifierInterner) -> Self {
        LexerParser {
            iter: code.chars().enumerate().peekable(),
            ids: interner,
            toks: vec![],
        }
    }
    pub fn parse(&mut self) -> Result<(), LexerError> {
        while let Some((at, c)) = self.iter.next() {
            let kind = match c {
                '"' => match parse_string(&mut self.iter) {
                    Some(s) => TokenKind::Literal(Literal::String(s)),
                    None => {
                        return Err(LexerError {
                            at,
                            message: "string literal has problem".to_string(),
                        });
                    }
                },
                c if c.is_whitespace() => continue,
                c if is_identifier_char::<true>(c) => {
                    let identifier = {
                        let mut str = String::from(c);
                        str.push_str(
                            CharsWhile::new(&mut self.iter, is_identifier_char::<false>)
                                .collect::<String>()
                                .as_str(),
                        );
                        str
                    };
                    if let Some(sym) = try_into_word_symbol(&identifier) {
                        TokenKind::WordSymbol(sym)
                    } else {
                        TokenKind::Identifier(self.ids.get_or_insert(identifier))
                    }
                }
                c if c.is_ascii_digit() || c == '-' || c == '+' => {
                    match try_iter_to_num(c, &mut self.iter) {
                        TryToNumstrResult::Smi(i) => TokenKind::Literal(Literal::Smi(i)),
                        TryToNumstrResult::Bigint(s) => TokenKind::Literal(Literal::BigInt(s)),
                        TryToNumstrResult::MinusSymbol => TokenKind::Symbol(Symbol::Minus),
                        TryToNumstrResult::PlusSymbol => TokenKind::Symbol(Symbol::Plus),
                        TryToNumstrResult::IncorrectSyntax(msg, at) => {
                            return Err(LexerError { at, message: msg });
                        }
                    }
                }
                c if let Some(sym) = try_into_symbol(c) => TokenKind::Symbol(sym),
                _ => {
                    return Err(LexerError {
                        at,
                        message: format!("unknown char: {c:?}").to_string(),
                    });
                }
            };
            self.toks.push(Token { kind, at });
        }
        Ok(())
    }
    pub fn take_toks(self) -> Vec<Token> {
        self.toks
    }
}
