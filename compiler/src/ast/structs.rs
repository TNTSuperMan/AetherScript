use crate::{
    ast::{func::Func, types::Type},
    interner::IdentifierId,
};
use std::collections::BTreeMap;

pub struct Struct {
    pub name: IdentifierId,
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub fields: BTreeMap<IdentifierId, Type>,
}

pub struct Enum {
    pub name: IdentifierId,
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub variants: BTreeMap<IdentifierId, EnumVariant>,
}
pub enum EnumVariant {
    Tuply(Vec<Type>),
    Structy(BTreeMap<IdentifierId, Type>),
}

pub struct Trait {
    pub name: IdentifierId,
    pub generics: Vec<Type>,
}

pub struct Impl {
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub impl_trait: Option<Trait>,
    pub target: Type,
    pub funcs: Vec<Func>,
}
