use crate::{
    ast::{
        AstError,
        parser::AstParser,
        stmt::{Block, If, Stmt, While},
    },
    lexer::{Symbol, Token, TokenKind, WordSymbol},
    next_tok,
};

impl AstParser {
    pub fn parse_block(&mut self, skip_openbrace: bool) -> Result<Block, AstError> {
        if !skip_openbrace {
            next_tok!(self.iter, TokenKind::Symbol(Symbol::OpenBrace) => {})?;
        }
        let mut stmts = vec![];
        loop {
            if let Some(Token {
                kind: TokenKind::Symbol(Symbol::CloseBrace),
                ..
            }) = self.iter.peek()
            {
                self.iter.next();
                break Ok(Block { stmts });
            } else {
                stmts.push(self.parse_stmt()?);
            }
        }
    }
    pub fn parse_stmt(&mut self) -> Result<Stmt, AstError> {
        let (i, t) = next_tok!(self.iter)?;
        match t {
            TokenKind::WordSymbol(WordSymbol::If) => {
                Ok(Stmt::If(If {
                    condition: self.parse_expr()?,
                    then: self.parse_block(false)?,
                    elses: None, // TODO: else
                }))
            }
            TokenKind::WordSymbol(WordSymbol::While) => Ok(Stmt::While(While {
                condition: self.parse_expr()?,
                body: self.parse_block(false)?,
            })),
            _ => AstError::todo("another stmt"),
        }
    }
}
