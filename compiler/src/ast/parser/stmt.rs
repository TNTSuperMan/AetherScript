use crate::{
    ast::{
        AstError,
        expr::Expr,
        parser::Parser,
        stmt::{Block, If, Stmt, While},
    },
    lexer::{Symbol, Token, TokenKind, WordSymbol},
    next_tok,
};

impl Parser for Block {
    fn parse(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Self, AstError> {
        next_tok!(iter, TokenKind::Symbol(Symbol::OpenBrace) => {})?;
        let mut stmts = vec![];
        loop {
            if let Some(Token {
                kind: TokenKind::Symbol(Symbol::CloseBrace),
                ..
            }) = iter.peek()
            {
                iter.next();
                break Ok(Block { stmts });
            } else {
                stmts.push(Stmt::parse(iter)?);
            }
        }
    }
}

impl Parser for Stmt {
    fn parse(iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Self, AstError> {
        let (i, t) = next_tok!(iter)?;
        match t {
            TokenKind::WordSymbol(WordSymbol::If) => {
                Ok(Stmt::If(If {
                    condition: Expr::parse(iter)?,
                    then: Block::parse(iter)?,
                    elses: None, // TODO: else
                }))
            }
            TokenKind::WordSymbol(WordSymbol::While) => Ok(Stmt::While(While {
                condition: Expr::parse(iter)?,
                body: Block::parse(iter)?,
            })),
            _ => AstError::todo("another stmt"),
        }
    }
}
