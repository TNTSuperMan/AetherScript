use crate::ast::{
    func::Func,
    structs::{Enum, Impl, Struct},
};

pub mod func;
pub mod structs;
pub mod types;

pub struct Module {
    pub statements: Vec<ModuleStatement>,
}

pub enum ModuleStatement {
    Func(Func),
    Struct(Struct),
    Enum(Enum),
    Impl(Impl),
}
