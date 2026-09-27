pub enum Type {
    Primitive(PrimitiveType),
    Impritive { mutable: bool, typ: ImprimitiveType },
}

pub enum PrimitiveType {
    Num,
    Smi,
    Str,
    Bool,
    Unique,
}

pub enum ImprimitiveType {
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Identifier { name: String, generics: Vec<Type> },
}
