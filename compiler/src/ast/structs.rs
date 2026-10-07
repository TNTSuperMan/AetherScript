use crate::{
    ast::{func::Func, types::Type},
    interner::IdentifierId,
};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Struct {
    pub name: IdentifierId,
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub fields: BTreeMap<IdentifierId, Type>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub name: IdentifierId,
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub variants: BTreeMap<IdentifierId, EnumVariant>,
}
#[derive(Clone, Debug)]
pub enum EnumVariant {
    Tuply(Vec<Type>),
    Structy(BTreeMap<IdentifierId, Type>),
}

#[derive(Clone, Debug)]
pub struct Trait {
    pub name: IdentifierId,
    pub generics: Vec<Type>,
}

#[derive(Clone, Debug)]
pub struct Impl {
    pub generics: Vec<(IdentifierId, Option<Trait>)>,
    pub impl_trait: Option<Trait>,
    pub target: Type,
    pub funcs: Vec<Func>,
}
