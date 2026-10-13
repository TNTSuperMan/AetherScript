use crate::ast::{AstError, expr::Expr, parser::AstParser};

impl AstParser {
    pub fn parse_expr(&mut self) -> Result<Expr, AstError> {
        AstError::todo("expr")
    }
}
