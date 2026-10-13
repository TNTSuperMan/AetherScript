use crate::{
    ast::{expr::Expr, types::Type},
    interner::IdentifierId,
};

#[derive(Clone, Debug)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Expr(Expr),
    DefineVariable(DefineVariable),
    Assign(Assign),
    If(If),
    While(While),
}

#[derive(Clone, Debug)]
pub struct DefineVariable {
    name: IdentifierId,
    mutable: bool,
    typ: Option<Type>,
    init: Expr,
}

#[derive(Clone, Debug)]
pub struct Assign {
    target: (),
    value: Expr,
}

#[derive(Clone, Debug)]
pub struct If {
    pub condition: Expr,
    pub then: Block,
    pub elses: Option<Block>,
}

#[derive(Clone, Debug)]
pub struct While {
    pub condition: Expr,
    pub body: Block,
}
