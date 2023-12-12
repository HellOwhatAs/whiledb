#[derive(Debug)]
pub enum Cmd {
    Asgn(Box<Expr>, Box<Expr>),
    Seq(Box<Cmd>, Box<Cmd>),
    If(Box<Expr>, Box<Cmd>, Box<Cmd>),
    While(Box<Expr>, Box<Cmd>),
    Expr(Box<Expr>),
    Continue,
    Break,
    Func(String, Option<Box<Expr>>, Box<Cmd>),
    Return(Box<Expr>),
    Nop
}

#[derive(Debug)]
pub enum Expr {
    Const(isize),
    Var(String),
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    UnOp(UnOp, Box<Expr>),
    Call(String, Option<Box<Expr>>),
    Tuple(Box<Expr>, Box<Expr>)
}

#[derive(Debug)]
pub enum BinOp { Plus, Minus, Mul, Div, Mod, Lt, Gt, Le, Ge, Eq, Ne, And, Or }

#[derive(Debug)]
pub enum UnOp { Negate, Not, Deref }