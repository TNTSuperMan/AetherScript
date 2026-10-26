use crate::{
    ast::{
        AstError,
        parser::{Parser, eof_err, error::err, parse_multi_tok},
        types::{GenericArg, ImprimitiveType, PrimitiveType, Type},
    },
    lexer::{Symbol, Token, TokenKind, WordSymbol},
    next_tok,
};

impl Parser for Type {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        let Token { at, kind } = iter.peek().ok_or_else(eof_err)?;
        match kind {
            TokenKind::WordSymbol(WordSymbol::Mut) => Ok(Type::Impritive {
                mutable: true,
                typ: ImprimitiveType::parse(iter)?,
            }),
            TokenKind::WordSymbol(w) => {
                let pt = match w {
                    WordSymbol::Smi => PrimitiveType::Smi,
                    WordSymbol::Float => PrimitiveType::Float,
                    WordSymbol::Bigint => PrimitiveType::Bigint,
                    WordSymbol::Str => PrimitiveType::Str,
                    WordSymbol::Bool => PrimitiveType::Bool,
                    WordSymbol::Unique => PrimitiveType::Unique,
                    WordSymbol::None => PrimitiveType::None,
                    _ => return err(*at, "unknown type"),
                };
                Ok(Type::Primitive(pt))
            }
            TokenKind::Identifier(_)
            | TokenKind::Symbol(Symbol::OpenBracket | Symbol::OpenBrace) => Ok(Type::Impritive {
                mutable: false,
                typ: ImprimitiveType::parse(iter)?,
            }),
            _ => err(*at, "unknown or not implemented type"),
        }
    }
}

impl Parser for ImprimitiveType {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        let Token { at, kind } = iter.next().ok_or_else(eof_err)?;
        Ok(match kind {
            TokenKind::Symbol(Symbol::OpenBracket) => {
                let typ = Type::parse(iter)?;
                ImprimitiveType::Array(Box::new(typ))
            }
            TokenKind::Symbol(Symbol::OpenParen) => {
                let elements =
                    parse_multi_tok(iter, Type::parse, Symbol::Comma, Symbol::CloseParen)?;
                ImprimitiveType::Tuple(elements)
            }
            TokenKind::Identifier(name) => {
                let generics = match iter.peek().ok_or_else(eof_err)?.kind {
                    TokenKind::Symbol(Symbol::Lt) => {
                        iter.next();
                        parse_multi_tok(iter, Type::parse, Symbol::Comma, Symbol::Gt)?
                    }
                    _ => vec![],
                };
                ImprimitiveType::Identifier { name, generics }
            }
            _ => return AstError::unex(at),
        })
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
