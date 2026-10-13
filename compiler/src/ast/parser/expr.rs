use crate::{
    ast::{AstError, expr::Expr, parser::AstParser},
    lexer::TokenKind,
    next_tok,
};

impl AstParser {
    pub fn parse_expr(&mut self) -> Result<Expr, AstError> {
        match next_tok!(self.iter)?.1 {
            TokenKind::Literal(literal) => Ok(Expr::Literal(literal)),
            _ => AstError::todo("another expr"),
        }
    }
}
