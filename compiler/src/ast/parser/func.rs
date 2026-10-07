use crate::{
    ast::{
        AstError,
        func::{Func, FuncArg, FuncAttr},
        parser::{AstParser, eof_err},
        types::GenericArg,
    },
    interner::IdentifierId,
    lexer, next_tok,
};

impl AstParser {
    fn parse_funcarg(&mut self) -> Result<FuncArg, AstError> {
        let (fat, ftok) = next_tok!(self.iter)?;
        let (name, mutable): (IdentifierId, bool) = match ftok {
            lexer::TokenKind::Identifier(name) => (name, false),
            lexer::TokenKind::WordSymbol(lexer::WordSymbol::Mut) => (
                next_tok!(self.iter, lexer::TokenKind::Identifier(name) => name)?.1,
                true,
            ),
            _ => {
                return Err(AstError {
                    at: fat,
                    message: "unknown tok during funcarg".to_string(),
                });
            }
        };
        next_tok!(self.iter, lexer::TokenKind::Symbol(lexer::Symbol::Colon) => {})?;

        Ok(FuncArg {
            mutable,
            name,
            var_type: self.parse_type()?,
        })
    }

    // 名前部分からパース
    pub fn parse_fn_base(
        &mut self,
        attributes: Vec<FuncAttr>,
        is_async: bool,
        is_unsafe: bool,
    ) -> Result<Func, AstError> {
        let (id_at, name) = next_tok!(self.iter, lexer::TokenKind::Identifier(name) => name)?;

        let generics: Vec<GenericArg> = match next_tok!(self.iter)? {
            (_, lexer::TokenKind::Symbol(lexer::Symbol::OpenParen)) => vec![],
            (_, lexer::TokenKind::Symbol(lexer::Symbol::Lt)) => todo!("generics"),
            (at, _) => {
                return Err(AstError {
                    at,
                    message: "unknown tok after funcname".to_string(),
                });
            }
        };

        let mut args = vec![];

        loop {
            let tok = self.iter.peek().ok_or_else(eof_err)?;
            match tok.kind {
                lexer::TokenKind::Symbol(lexer::Symbol::CloseParen) => {
                    self.iter.next();
                    break;
                }
                lexer::TokenKind::WordSymbol(lexer::WordSymbol::Mut)
                | lexer::TokenKind::Identifier(_) => {
                    args.push(self.parse_funcarg()?);
                    todo!()
                }
                _ => {
                    return Err(AstError {
                        at: tok.at,
                        message: "unknown tok during args".to_string(),
                    });
                }
            }
        }

        Ok(Func {
            attributes,
            is_async,
            is_unsafe,
            name,
            generics,
            self_arg: None,
            args,
            return_type: todo!(),
            body: todo!(),
        })
    }
}
