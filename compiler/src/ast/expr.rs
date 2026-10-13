use crate::lexer::Literal;

#[derive(Clone, Debug)]
pub enum Expr {
    Literal(Literal),
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    Unary {
        kind: UnaryExprKind,
        expr: Box<Expr>,
    },
    Binary {
        kind: BinaryExprKind,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum UnaryExprKind {
    BitNot,
    BoolNot,
}

#[derive(Clone, Debug)]
pub enum BinaryExprKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
    BitOr,
    BitAnd,
    BitXor,
    BoolOr,
    BoolAnd,
    BolXor,
    Eq,
    Lt,
    Gt,
}
