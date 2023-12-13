use std::collections::VecDeque;

#[derive(Debug)]
pub enum Cmd {
    Asgn(Box<Expr>, Box<Expr>),
    Seq(VecDeque<Cmd>),
    If(Box<Expr>, Box<Cmd>, Box<Cmd>),
    While(Box<Expr>, Box<Cmd>),
    Expr(Box<Expr>),
    Continue,
    Break,
    Func(String, Box<Expr>, Box<Cmd>),
    Class(String, Box<Cmd>),
    Return(Box<Expr>),
    Nop
}

#[derive(Debug)]
pub enum Expr {
    ConstInt(String),
    ConstFloat(String),
    Tuple(VecDeque<Expr>),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Call(Box<Expr>, Box<Expr>),
    GetItem(Box<Expr>, Box<Expr>),
    GetAttr(Box<Expr>, String),
}

#[derive(Debug)]
pub enum BinOp { Plus, Minus, Mul, Div, Mod, Lt, Gt, Le, Ge, Eq, Ne, And, Or }

#[derive(Debug)]
pub enum UnOp { Negate, Not, Deref }