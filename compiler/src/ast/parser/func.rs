use crate::{
    ast::{
        AstError,
        func::{Func, FuncArg, FuncAttr},
        parser::AstParser,
        types::GenericArg,
    },
    lexer, next_tok,
};

impl AstParser {
    fn parse_funcarg(&mut self) -> Result<FuncArg, AstError> {
        next_tok!(self.iter, lexer::TokenKind::Symbol(lexer::Symbol::Colon) => {})?;

        todo!()
    }

    // 名前部分からパース
    pub fn parse_fn_base(
        &mut self,
        attributes: Vec<FuncAttr>,
        is_async: bool,
        is_unsafe: bool,
    ) -> Result<Func, AstError> {
        let (id_at, name) = next_tok!(self.iter, lexer::TokenKind::Identifier(name) => name)?;

        let generics: Vec<GenericArg> = match next_tok!(self.iter)? {
            (_, lexer::TokenKind::Symbol(lexer::Symbol::OpenParen)) => vec![],
            (_, lexer::TokenKind::Symbol(lexer::Symbol::Lt)) => todo!("generics"),
            (at, _) => {
                return Err(AstError {
                    at,
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
