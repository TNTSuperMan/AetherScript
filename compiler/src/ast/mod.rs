use crate::{
    ast::{
        func::Func,
        parser::AstParser,
        structs::{Enum, Impl, Struct},
    },
    lexer,
};
pub use parser::error::AstError;

pub mod func;
mod parser;
pub mod structs;
pub mod types;

pub fn parse_to_ast(lexer: Vec<lexer::Token>) -> Result<Module, AstError> {
    let mut parser = AstParser::new(lexer);
    parser.parse()?;
    Ok(parser.take_ast())
}

pub struct Module {
    pub statements: Vec<ModuleStatement>,
}

pub enum ModuleStatement {
    Func(Func),
    Struct(Struct),
    Enum(Enum),
    Impl(Impl),
}
