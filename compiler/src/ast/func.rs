use crate::{
    ast::{
        stmt::Block,
        types::{GenericArg, Type},
    },
    interner::IdentifierId,
};

#[derive(Clone, Debug)]
pub struct Func {
    pub attributes: Vec<FuncAttr>,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub name: IdentifierId,
    pub generics: Vec<GenericArg>,
    pub self_arg: Option<bool>,
    pub args: Vec<FuncArg>,
    pub return_type: Type,
    pub body: Block,
}

#[derive(Clone, Debug)]
pub enum FuncAttr {
    Inline(FuncInlineAttr),
    Unsafe,
    Impure,
}
#[derive(Clone, Debug)]
pub enum FuncInlineAttr {
    Always,
    Auto,
    Never,
}
#[derive(Clone, Debug)]
pub enum FuncUnsafeAttr {
    Pure,
}

#[derive(Clone, Debug)]
pub struct FuncArg {
    pub mutable: bool,
    pub name: IdentifierId,
    pub var_type: Type,
}
