use std::collections::VecDeque;
use std::rc::Rc;


/// Command Node in AST
#[derive(Debug)]
pub enum Cmd {
    /// - `"cmd_simple"`  
    ///   represent the assign syntax
    ///   ```
    ///   "expr" = "expr"
    ///   ```
    Asgn(Rc<Expr>, Rc<Expr>),
    /// - `"cmd"`  
    ///   represent a sequence of "cmd_simple" or "cmd_block"
    /// - `"fn_list"`  
    ///   represent a sequence of "fn_block" defined in "class_block"
    Seq(VecDeque<Rc<Cmd>>),
    /// - `"cmd_block"`  
    ///   represent the if block
    ///   ```
    ///   if "expr" /* then */ {
    ///       "cmd"
    ///   }
    ///   /* else {
    ///       "cmd"
    ///   } */
    ///   ```
    If(Rc<Expr>, Rc<Cmd>, Rc<Cmd>),
    /// - `"cmd_block"`  
    ///   represent the while block
    ///   ```
    ///   while "expr" /* do */ {
    ///       "cmd"
    ///   }
    ///   ```
    While(Rc<Expr>, Rc<Cmd>),
    /// - `"cmd_simple"`  
    ///   represent the command that is just a value
    ///   ```
    ///   "expr"
    ///   ```
    /// - Wrap type `Expr` to type `Cmd`
    Expr(Rc<Expr>),
    /// - `"cmd_simple"`  
    ///   ```
    ///   continue
    ///   ```
    Continue,
    /// - `"cmd_simple"`  
    ///   ```
    ///   break
    ///   ```
    Break,
    /// - `"fn_block"`  
    ///   defination of a function (or method of a class)
    ///   ```
    ///   fn ident("ident_list") {
    ///       "cmd"
    ///   }
    ///   ```
    Func(String, Rc<Expr>, Rc<Cmd>),
    /// - `"class_block"`  
    ///   defination of a class
    ///   ```
    ///   class ident {
    ///       "fn_list"
    ///   }
    ///   ```
    Class(String, Rc<Cmd>),
    /// - `"cmd_simple"`  
    ///   ```
    ///   return "expr"
    ///   ```
    Return(Rc<Expr>),
    /// - No operation
    Nop
}

/// Expression Node in AST
#[derive(Debug)]
pub enum Expr {
    /// - `"int"`  
    ///   literal int matched by `0|[1-9][0-9]*`
    ConstInt(String),
    /// - `"float"`  
    ///   literal float matched by `\d+\.\d+`
    ConstFloat(String),
    /// - `"string"`  
    ///   literal string matched by `"(?:\\.|[^\\"])*"`
    ConstString(String),
    /// - `"ident_list"`  
    ///   formal parameter list in `"fn_block"`
    /// - `"expr_list"`  
    ///   - array in source code  
    ///     ```
    ///     ["expr", "expr"/*, */]
    ///     ```
    ///   - real parameter list when calling a function  
    ///     ```
    ///     "expr"("expr", "expr"/*, */)
    ///     ```
    ///   - index list when getting items  
    ///     ```
    ///     "expr"["expr", "expr"/*, */]
    ///     ```
    Tuple(VecDeque<Rc<Expr>>),
    /// - `"ident"`  
    ///   variable
    Var(String),
    /// - `"expr"`  
    ///   binary operation
    ///   ```
    ///   "expr" op "expr"
    ///   ```
    BinOp(BinOp, Rc<Expr>, Rc<Expr>),
    /// - `"expr"`  
    ///   unary operation
    ///   ```
    ///   op "expr"
    ///   ```
    UnOp(UnOp, Rc<Expr>),
    /// - `"expr"`  
    ///   call a function
    ///   ```
    ///   "expr"("expr", "expr"/*, */)
    ///   ```
    Call(Rc<Expr>, Rc<Expr>),
    /// - `"expr"`  
    ///   get item from `"expr"`
    ///   ```
    ///   "expr"["expr", "expr"/*, */]
    ///   ```
    GetItem(Rc<Expr>, Rc<Expr>),
    /// - `"expr"`  
    ///   get attribute
    ///   ```
    ///   "expr"."ident"
    ///   ```
    GetAttr(Rc<Expr>, String),
}

/// Binary Operators
#[derive(Debug)]
pub enum BinOp {
    /// operator `+`
    Plus,
    /// operator `-`
    Minus,
    /// operator `*`
    Mul,
    /// operator `/`
    Div,
    /// operator `%`
    Mod,
    /// operator `<`
    Lt,
    /// operator `>`
    Gt,
    /// operator `<=`
    Le,
    /// operator `>=`
    Ge,
    /// operator `==`
    Eq,
    /// operator `!=`
    Ne,
    /// operator `&&`
    And,
    /// operator `||`
    Or,
}

impl std::fmt::Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BinOp::Plus => "add",
            BinOp::Minus => "sub",
            BinOp::Mul => "mul",
            BinOp::Div => "div",
            BinOp::Mod => "mod",
            BinOp::Lt => "lt",
            BinOp::Gt => "gt",
            BinOp::Le => "le",
            BinOp::Ge => "ge",
            BinOp::Eq => "eq",
            BinOp::Ne => "ne",
            BinOp::And => "and",
            BinOp::Or => "or",
        })
    }
}

/// Unary Operators
#[derive(Debug)]
pub enum UnOp {
    /// operator `-`
    Negate,
    /// operator `!`
    Not, 
    /// operator `*`
    Deref
}

impl std::fmt::Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            UnOp::Negate => "negate",
            UnOp::Not => "not",
            UnOp::Deref => "deref",
        })
    }
}