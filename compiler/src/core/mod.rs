use crate::{
    ast::{self, parse_to_ast},
    lexer::{interner::IdentifierInterner, parse_to_lexer},
};
use std::fs;

pub fn load_file_to_ast(
    interner: &mut IdentifierInterner,
    path: &str,
) -> Result<ast::Module, String> {
    let code = fs::read_to_string(path).map_err(|err| format!("failed to read `{path}`: {err}"))?;
    let lex = parse_to_lexer(&code, interner).map_err(|err| err.to_error_msg(&code, path))?;
    parse_to_ast(lex).map_err(|err| err.to_error_msg(&code, path))
}

pub fn start_compile(entrypoint: &str) -> Result<(), String> {
    let mut interner = IdentifierInterner::new();

    let entry_ast = load_file_to_ast(&mut interner, entrypoint)?;

    Err("todo".to_string())
}
