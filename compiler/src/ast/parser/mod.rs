use crate::{
    ast::{Module, ModuleStatement, func::Func, parser::error::AstError},
    lexer::{Symbol, Token, TokenKind, WordSymbol},
};
use std::{iter::Peekable, vec::IntoIter};

pub mod error;
mod expr;
mod func;
mod stmt;
mod types;

pub(super) fn eof_err() -> AstError {
    AstError {
        at: usize::MAX,
        message: "eof reached during syntax".to_string(),
    }
}

pub trait Parser: Sized {
    fn parse(iter: &mut Peekable<IntoIter<Token>>) -> Result<Self, AstError>;
}

pub fn parse_multi_tok<T, F: FnMut(&mut Peekable<IntoIter<Token>>) -> Result<T, AstError>>(
    iter: &mut Peekable<IntoIter<Token>>,
    mut f: F,
    sep: Symbol,
    end: Symbol,
) -> Result<Vec<T>, AstError> {
    let sep = TokenKind::Symbol(sep);
    let end = TokenKind::Symbol(end);
    let mut toks = vec![];

    if &iter.peek().ok_or_else(eof_err)?.kind == &end {
        Ok(toks)
    } else {
        loop {
            toks.push(f(iter)?);
            let sep_tok = iter.next().ok_or_else(eof_err)?;
            if &sep_tok.kind == &sep {
                continue;
            }
            if &sep_tok.kind == &end {
                break Ok(toks);
            }
            break AstError::unex(sep_tok.at);
        }
    }
}

#[macro_export]
macro_rules! next_tok {
    ($iter: expr) => {
        match $iter.next() {
            Some($crate::lexer::Token { at, kind }) => Ok((at, kind)),
            None => Err($crate::ast::parser::eof_err()),
        }
    };
    ($iter: expr, $($pat: pat => $expr: expr),+) => {
        next_tok!(
            @ $iter,
            format!("{} token expected", stringify!($($pat)or+)),
            $($pat => $expr),+
        )
    };
    (@ $iter: expr, $errmsg: expr, $($pat: pat => $expr: expr),+) => {
        next_tok!($iter).and_then(|(at, kind)| match kind {
            $(
                $pat => Ok((at, $expr)),
            ),+
            _ => Err(AstError {
                at,
                message: $errmsg,
            }),
        })
    };
}

pub(crate) struct AstParser {
    iter: Peekable<IntoIter<Token>>,
    module: Module,
}

impl AstParser {
    pub fn new(lexer: Vec<Token>) -> Self {
        Self {
            iter: lexer.into_iter().peekable(),
            module: Module { statements: vec![] },
        }
    }
    pub fn parse(&mut self) -> Result<(), AstError> {
        while self.iter.peek().is_some() {
            self.module
                .statements
                .push(ModuleStatement::parse(&mut self.iter)?);
        }
        Ok(())
    }
    pub fn take_ast(self) -> Module {
        self.module
    }
}

impl Parser for ModuleStatement {
    fn parse(iter: &mut Peekable<IntoIter<Token>>) -> Result<Self, AstError> {
        let tok = iter.peek().ok_or_else(eof_err)?;
        match tok.kind {
            TokenKind::WordSymbol(WordSymbol::Function) => {
                Ok(ModuleStatement::Func(Func::parse(iter)?))
            }
            TokenKind::WordSymbol(WordSymbol::Struct) => AstError::todo("struct"),
            TokenKind::WordSymbol(WordSymbol::Enum) => AstError::todo("enum"),
            TokenKind::WordSymbol(WordSymbol::Impl) => AstError::todo("impl"),
            _ => AstError::unex(tok.at),
        }
    }
}
