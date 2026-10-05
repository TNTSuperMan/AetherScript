use crate::interner::IdentifierId;

pub enum Type {
    Primitive(PrimitiveType),
    Impritive { mutable: bool, typ: ImprimitiveType },
}

pub enum PrimitiveType {
    Smi,
    Float,
    Bigint,
    Str,
    Bool,
    Unique,
}

pub enum ImprimitiveType {
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Identifier {
        name: IdentifierId,
        generics: Vec<Type>,
    },
}

pub struct GenericArg {
    pub name: IdentifierId,
    pub base: Option<(IdentifierId, Vec<Type>)>,
}
