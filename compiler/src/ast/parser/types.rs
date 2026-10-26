use crate::{
    ast::{
        AstError,
        parser::{Parser, eof_err, error::err},
        types::{GenericArg, PrimitiveType, Type},
    },
    lexer::{Symbol, TokenKind, WordSymbol},
    next_tok,
};

impl Parser for Type {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        let (at, tok) = next_tok!(iter)?;
        match tok {
            TokenKind::WordSymbol(w) => {
                let pt = match w {
                    WordSymbol::Smi => PrimitiveType::Smi,
                    WordSymbol::Float => PrimitiveType::Float,
                    WordSymbol::Bigint => PrimitiveType::Bigint,
                    WordSymbol::Str => PrimitiveType::Str,
                    WordSymbol::Bool => PrimitiveType::Bool,
                    WordSymbol::Unique => PrimitiveType::Unique,
                    WordSymbol::None => PrimitiveType::None,
                    _ => return err(at, "unknown type"),
                };
                Ok(Type::Primitive(pt))
            }
            _ => err(at, "unknown or not implemented type"),
        }
    }
}

impl Parser for GenericArg {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        let (_, name) = next_tok!(iter, TokenKind::Identifier(id) => id)?;

        let base = match iter.peek().ok_or_else(eof_err)?.tuple() {
            (_, TokenKind::Symbol(Symbol::Colon)) => {
                iter.next();
                Some(Type::parse(iter)?)
            }
            _ => None,
        };
        Ok(GenericArg { name, base })
    }
}
