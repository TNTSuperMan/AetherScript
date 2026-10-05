use crate::{
    ast::types::{GenericArg, Type},
    interner::IdentifierId,
};

pub struct Func {
    pub attributes: Vec<FuncAttr>,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub name: IdentifierId,
    pub generics: Vec<GenericArg>,
    pub self_arg: Option<bool>,
    pub args: Vec<FuncArg>,
    pub return_type: Type,
    pub body: Vec<FuncStatement>,
}

pub enum FuncAttr {
    Inline(FuncInlineAttr),
    Unsafe,
    Impure,
}
pub enum FuncInlineAttr {
    Always,
    Auto,
    Never,
}
pub enum FuncUnsafeAttr {
    Pure,
}

pub struct FuncArg {
    pub mutable: bool,
    pub name: IdentifierId,
    pub var_type: Type,
}

pub enum FuncStatement {}
