use crate::{
    ast::{
        AstError,
        parser::{AstParser, error::err},
        types::{PrimitiveType, Type},
    },
    lexer::{TokenKind, WordSymbol},
    next_tok,
};

impl AstParser {
    pub fn parse_type(&mut self) -> Result<Type, AstError> {
        let (at, tok) = next_tok!(self.iter)?;
        match tok {
            TokenKind::WordSymbol(w) => {
                let pt = match w {
                    WordSymbol::Smi => PrimitiveType::Smi,
                    WordSymbol::Float => PrimitiveType::Float,
                    WordSymbol::Bigint => PrimitiveType::Bigint,
                    WordSymbol::Str => PrimitiveType::Str,
                    WordSymbol::Bool => PrimitiveType::Bool,
                    WordSymbol::Unique => PrimitiveType::Unique,
                    _ => return err(at, "unknown type"),
                };
                Ok(Type::Primitive(pt))
            }
            _ => err(at, "unknown or not implemented type"),
        }
    }
}
