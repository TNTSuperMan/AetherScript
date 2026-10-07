use crate::interner::IdentifierId;

#[derive(Clone, Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    Impritive { mutable: bool, typ: ImprimitiveType },
}

#[derive(Clone, Debug)]
pub enum PrimitiveType {
    Smi,
    Float,
    Bigint,
    Str,
    Bool,
    Unique,
}

#[derive(Clone, Debug)]
pub enum ImprimitiveType {
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Identifier {
        name: IdentifierId,
        generics: Vec<Type>,
    },
}

#[derive(Clone, Debug)]
pub struct GenericArg {
    pub name: IdentifierId,
    pub base: Option<(IdentifierId, Vec<Type>)>,
}
