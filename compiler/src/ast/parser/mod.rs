use crate::{
    ast::{Module, ModuleStatement, parser::error::AstError},
    lexer::{Token, TokenKind, WordSymbol},
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
        while let Some(tok) = self.iter.next() {
            let stmt = match tok.kind {
                TokenKind::WordSymbol(w) => match w {
                    WordSymbol::Function => {
                        ModuleStatement::Func(self.parse_fn_base(vec![], false, false)?)
                    }
                    WordSymbol::Struct => return AstError::todo("struct"),
                    WordSymbol::Enum => return AstError::todo("enum"),
                    WordSymbol::Impl => return AstError::todo("impl"),
                    _ => return AstError::unex(tok.at),
                },
                _ => return AstError::unex(tok.at),
            };
            self.module.statements.push(stmt);
        }
        Ok(())
    }
    pub fn take_ast(self) -> Module {
        self.module
    }
}
