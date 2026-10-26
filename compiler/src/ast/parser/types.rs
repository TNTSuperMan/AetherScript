use crate::{
    ast::{
        AstError,
        parser::{Parser, error::err},
        types::{PrimitiveType, Type},
    },
    lexer::{TokenKind, WordSymbol},
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
