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
    Return(Box<Expr>),
    Nop
}

#[derive(Debug)]
pub enum Expr {
    Const(isize),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Call(String, Box<Expr>),
    Tuple(VecDeque<Expr>)
}

#[derive(Debug)]
pub enum BinOp { Plus, Minus, Mul, Div, Mod, Lt, Gt, Le, Ge, Eq, Ne, And, Or }

#[derive(Debug)]
pub enum UnOp { Negate, Not, Deref }