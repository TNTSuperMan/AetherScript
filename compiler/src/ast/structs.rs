use crate::ast::{func::Func, types::Type};
use std::collections::BTreeMap;

pub struct Struct {
    pub name: String,
    pub generics: Vec<(String, Option<Trait>)>,
    pub fields: BTreeMap<String, Type>,
}

pub struct Enum {
    pub name: String,
    pub generics: Vec<(String, Option<Trait>)>,
    pub variants: BTreeMap<String, EnumVariant>,
}
pub enum EnumVariant {
    Tuply(Vec<Type>),
    Structy(BTreeMap<String, Type>),
}

pub struct Trait {
    pub name: String,
    pub generics: Vec<Type>,
}

pub struct Impl {
    pub generics: Vec<(String, Option<Trait>)>,
    pub impl_trait: Option<Trait>,
    pub target: Type,
    pub funcs: Vec<Func>,
}
