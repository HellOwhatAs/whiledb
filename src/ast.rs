use std::collections::VecDeque;

/// Command Node in AST
#[derive(Debug)]
pub enum Cmd {
    /// - `"cmd_simple"`  
    ///   represent the assign syntax
    ///   ```
    ///   "expr" = "expr"
    ///   ```
    Asgn(Box<Expr>, Box<Expr>),
    /// - `"cmd"`  
    ///   represent a sequence of "cmd_simple" or "cmd_block"
    /// - `"fn_list"`  
    ///   represent a sequence of "fn_block" defined in "class_block"
    Seq(VecDeque<Cmd>),
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
    If(Box<Expr>, Box<Cmd>, Box<Cmd>),
    /// - `"cmd_block"`  
    ///   represent the while block
    ///   ```
    ///   while "expr" /* do */ {
    ///       "cmd"
    ///   }
    ///   ```
    While(Box<Expr>, Box<Cmd>),
    /// - `"cmd_simple"`  
    ///   represent the command that is just a value
    ///   ```
    ///   "expr"
    ///   ```
    /// - Wrap type `Expr` to type `Cmd`
    Expr(Box<Expr>),
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
    Func(String, Box<Expr>, Box<Cmd>),
    /// - `"class_block"`  
    ///   defination of a class
    ///   ```
    ///   class ident {
    ///       "fn_list"
    ///   }
    ///   ```
    Class(String, Box<Cmd>),
    /// - `"cmd_simple"`  
    ///   ```
    ///   return "expr"
    ///   ```
    Return(Box<Expr>),
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
    Tuple(VecDeque<Expr>),
    /// - `"ident"`  
    ///   variable
    Var(String),
    /// - `"expr"`  
    ///   binary operation
    ///   ```
    ///   "expr" op "expr"
    ///   ```
    BinOp(BinOp, Box<Expr>, Box<Expr>),
    /// - `"expr"`  
    ///   unary operation
    ///   ```
    ///   op "expr"
    ///   ```
    UnOp(UnOp, Box<Expr>),
    /// - `"expr"`  
    ///   call a function
    ///   ```
    ///   "expr"("expr", "expr"/*, */)
    ///   ```
    Call(Box<Expr>, Box<Expr>),
    /// - `"expr"`  
    ///   get item from `"expr"`
    ///   ```
    ///   "expr"["expr", "expr"/*, */]
    ///   ```
    GetItem(Box<Expr>, Box<Expr>),
    /// - `"expr"`  
    ///   get attribute
    ///   ```
    ///   "expr"."ident"
    ///   ```
    GetAttr(Box<Expr>, String),
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