use crate::{
    ast::{
        AstError,
        parser::AstParser,
        types::{PrimitiveType, Type},
    },
    lexer::{self, TokenKind, WordSymbol},
    next_tok,
};

impl AstParser {
    pub fn parse_type(&mut self) -> Result<Type, AstError> {
        let (fat, ftok) = next_tok!(self.iter)?;
        match ftok {
            TokenKind::WordSymbol(w) => {
                let pt = match w {
                    WordSymbol::Smi => PrimitiveType::Smi,
                    WordSymbol::Float => PrimitiveType::Float,
                    WordSymbol::Bigint => PrimitiveType::Bigint,
                    WordSymbol::Str => PrimitiveType::Str,
                    WordSymbol::Bool => PrimitiveType::Bool,
                    WordSymbol::Unique => PrimitiveType::Unique,
                    _ => {
                        return Err(AstError {
                            at: fat,
                            message: "unknown type".to_string(),
                        });
                    }
                };
                Ok(Type::Primitive(pt))
            }
            _ => Err(AstError {
                at: fat,
                message: "unknown or implemented type".to_string(),
            }),
        }
    }
}
