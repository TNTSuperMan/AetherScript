use crate::{
    ast::{AstError, parser::AstParser, stmt::Stmt},
    lexer::{TokenKind, WordSymbol},
    next_tok,
};

impl AstParser {
    pub fn parse_stmt(&mut self) -> Result<Stmt, AstError> {
        let (i, t) = next_tok!(self.iter)?;
        match t {
            TokenKind::WordSymbol(WordSymbol::If) => {
                let condition = self.parse_expr()?;
                return AstError::todo("if body");
            }
            TokenKind::WordSymbol(WordSymbol::While) => {
                let condition = self.parse_expr()?;
                return AstError::todo("while body");
            }
            _ => todo!(),
        }
        AstError::todo("expr")
    }
}
