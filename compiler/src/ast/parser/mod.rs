use crate::{
    ast::{self, ModuleStatement, parser::error::AstError},
    lexer,
};
use std::{iter::Peekable, vec::IntoIter};

pub mod error;
mod func;

pub(crate) struct AstParser {
    iter: Peekable<IntoIter<lexer::Token>>,
    module: ast::Module,
}

impl AstParser {
    pub fn new(lexer: Vec<lexer::Token>) -> Self {
        Self {
            iter: lexer.into_iter().peekable(),
            module: ast::Module { statements: vec![] },
        }
    }
    pub fn parse(&mut self) -> Result<(), AstError> {
        while let Some(tok) = self.iter.next() {
            let stmt = match tok.kind {
                lexer::TokenKind::WordSymbol(w) => match w {
                    lexer::WordSymbol::Function => {
                        ModuleStatement::Func(self.parse_fn_base(tok.at, vec![], false, false)?)
                    }
                    lexer::WordSymbol::Struct => todo!(),
                    lexer::WordSymbol::Enum => todo!(),
                    lexer::WordSymbol::Impl => todo!(),
                    _ => {
                        return Err(AstError {
                            at: tok.at,
                            message: "invalid word symbol".to_string(),
                        });
                    }
                },
                _ => {
                    return Err(AstError {
                        at: tok.at,
                        message: "invalid token".to_string(),
                    });
                }
            };
            self.module.statements.push(stmt);
        }
        Ok(())
    }
    pub fn take_ast(self) -> ast::Module {
        self.module
    }
}
