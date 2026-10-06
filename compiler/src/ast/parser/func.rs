use crate::{
    ast::{
        AstError,
        func::{Func, FuncAttr},
        parser::AstParser,
        types::GenericArg,
    },
    lexer,
};

impl AstParser {
    // 名前部分からパース
    pub fn parse_fn_base(
        &mut self,
        begin_at: usize,
        attributes: Vec<FuncAttr>,
        is_async: bool,
        is_unsafe: bool,
    ) -> Result<Func, AstError> {
        let Some(lexer::Token {
            at: id_at,
            kind: lexer::TokenKind::Identifier(name),
        }) = self.iter.next()
        else {
            return Err(AstError {
                at: begin_at,
                message: "func name nothing".to_string(),
            });
        };

        let lexer::Token {
            at: nx_at,
            kind: after_id_tok,
        } = self.iter.next().ok_or_else(|| AstError {
            at: id_at,
            message: "eof detected during func".to_string(),
        })?;

        let generics: Vec<GenericArg> = match after_id_tok {
            lexer::TokenKind::Symbol(lexer::Symbol::OpenParen) => vec![],
            lexer::TokenKind::Symbol(lexer::Symbol::Lt) => todo!("generics"),
            _ => {
                return Err(AstError {
                    at: nx_at,
                    message: "unknown tok after funcname".to_string(),
                });
            }
        };

        Ok(Func {
            attributes,
            is_async,
            is_unsafe,
            name,
            generics,
            self_arg: todo!(),
            args: todo!(),
            return_type: todo!(),
            body: todo!(),
        })
    }
}
