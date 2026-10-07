use crate::{
    ast::{
        AstError,
        func::{Func, FuncArg, FuncAttr},
        parser::{AstParser, eof_err},
        types::GenericArg,
    },
    interner::IdentifierId,
    lexer::{Symbol, TokenKind, WordSymbol},
    next_tok,
};

impl AstParser {
    fn parse_funcarg(&mut self) -> Result<FuncArg, AstError> {
        let (at, tok) = next_tok!(self.iter)?;
        let (name, mutable): (IdentifierId, bool) = match tok {
            TokenKind::Identifier(name) => (name, false),
            TokenKind::WordSymbol(WordSymbol::Mut) => (
                next_tok!(self.iter, TokenKind::Identifier(name) => name)?.1,
                true,
            ),
            _ => return AstError::unex(at),
        };
        next_tok!(self.iter, TokenKind::Symbol(Symbol::Colon) => {})?;

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
        let (_, name) = next_tok!(self.iter, TokenKind::Identifier(name) => name)?;

        let generics: Vec<GenericArg> = match next_tok!(self.iter)? {
            (_, TokenKind::Symbol(Symbol::OpenParen)) => vec![],
            (_, TokenKind::Symbol(Symbol::Lt)) => return AstError::todo("generics"),
            (at, _) => return AstError::unex(at),
        };

        let mut args = vec![];

        loop {
            let tok = self.iter.peek().ok_or_else(eof_err)?;
            match tok.kind {
                TokenKind::Symbol(Symbol::CloseParen) => {
                    self.iter.next();
                    break;
                }
                TokenKind::WordSymbol(WordSymbol::Mut) | TokenKind::Identifier(_) => {
                    args.push(self.parse_funcarg()?);
                    return AstError::todo("args");
                }
                _ => return AstError::unex(tok.at),
            }
        }
        next_tok!(self.iter, TokenKind::Symbol(Symbol::Colon) => {})?;
        let return_type = self.parse_type()?;

        Ok(Func {
            attributes,
            is_async,
            is_unsafe,
            name,
            generics,
            self_arg: None,
            args,
            return_type,
            body: vec![],
        })
    }
}
