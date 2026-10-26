use crate::{
    ast::{
        AstError,
        func::{Func, FuncArg},
        parser::{Parser, eof_err},
        stmt::Block,
        types::{GenericArg, PrimitiveType, Type},
    },
    interner::IdentifierId,
    lexer::{Symbol, TokenKind, WordSymbol},
    next_tok,
};

impl Parser for FuncArg {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        let (at, tok) = next_tok!(iter)?;
        let (name, mutable): (IdentifierId, bool) = match tok {
            TokenKind::Identifier(name) => (name, false),
            TokenKind::WordSymbol(WordSymbol::Mut) => (
                next_tok!(iter, TokenKind::Identifier(name) => name)?.1,
                true,
            ),
            _ => return AstError::unex(at),
        };
        next_tok!(iter, TokenKind::Symbol(Symbol::Colon) => {})?;

        Ok(FuncArg {
            mutable,
            name,
            var_type: Type::parse(iter)?,
        })
    }
}

impl Parser for Func {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        next_tok!(iter, TokenKind::WordSymbol(WordSymbol::Function) => {})?;
        let (_, name) = next_tok!(iter, TokenKind::Identifier(name) => name)?;

        let generics: Vec<GenericArg> = match next_tok!(iter)? {
            (_, TokenKind::Symbol(Symbol::OpenParen)) => vec![],
            (_, TokenKind::Symbol(Symbol::Lt)) => return AstError::todo("generics"),
            (at, _) => return AstError::unex(at),
        };

        let mut args = vec![];

        loop {
            let tok = iter.peek().ok_or_else(eof_err)?;
            match tok.kind {
                TokenKind::Symbol(Symbol::CloseParen) => {
                    iter.next();
                    break;
                }
                TokenKind::WordSymbol(WordSymbol::Mut) | TokenKind::Identifier(_) => {
                    args.push(FuncArg::parse(iter)?);

                    match next_tok!(iter)? {
                        (_, TokenKind::Symbol(Symbol::CloseParen)) => break,
                        (_, TokenKind::Symbol(Symbol::Dot)) => continue,
                        (at, _) => return AstError::unex(at),
                    }
                }
                _ => return AstError::unex(tok.at),
            }
        }
        let return_type = match next_tok!(iter)? {
            (_, TokenKind::Symbol(Symbol::Colon)) => {
                let typ = Type::parse(iter)?;
                next_tok!(iter, TokenKind::Symbol(Symbol::OpenBrace) => {})?;
                typ
            }
            (_, TokenKind::Symbol(Symbol::OpenBrace)) => Type::Primitive(PrimitiveType::None),
            (i, _) => return AstError::unex(i),
        };

        Ok(Func {
            attributes: vec![],
            is_async: false,
            is_unsafe: false,
            name,
            generics,
            self_arg: None,
            args,
            return_type,
            body: todo!(), // Block::parse(iter)?
        })
    }
}
