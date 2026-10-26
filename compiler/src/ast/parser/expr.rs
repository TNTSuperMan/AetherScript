use crate::{
    ast::{AstError, expr::Expr, parser::Parser},
    lexer::TokenKind,
    next_tok,
};

impl Parser for Expr {
    fn parse(
        iter: &mut std::iter::Peekable<std::vec::IntoIter<crate::lexer::Token>>,
    ) -> Result<Self, AstError> {
        match next_tok!(iter)?.1 {
            TokenKind::Literal(literal) => Ok(Expr::Literal(literal)),
            _ => AstError::todo("another expr"),
        }
    }
}
