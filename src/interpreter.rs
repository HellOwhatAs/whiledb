pub mod runner;
pub mod utils;
pub mod states;
pub mod obj_type;
pub mod obj_int;
pub mod obj_bool;
pub mod obj_string;
pub mod obj_list;
use crate::ast::*;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::rc::Rc;
use num::BigInt;
use maplit;
use anyhow::Result;
#[cfg(not(debug_assertions))]
use anyhow::bail;
#[cfg(debug_assertions)]
use panic as bail;
pub use runner::{eval, exec};
pub use states::init_state;

#[derive(Debug)]
pub enum BuildIn {
    Bool(bool),
    Int(BigInt),
    Float(f64),
    String(String),
    Tuple(VecDeque<Any>),
    Not
}

#[derive(Debug)]
pub struct Object {
    buildin: BuildIn,
    attrs: HashMap<String, Any>
}

#[derive(Debug)]
pub struct DefinedFunction {
    args: Rc<Expr>,
    body: Rc<Cmd>
}

#[derive(Debug)]
pub struct BuildInFunction(fn(&VecDeque<Any>, Any) -> Result<Any>);

#[derive(Debug)]
pub enum Function {
    BuildInFunction(BuildInFunction),
    DefinedFunction(DefinedFunction)
}

#[derive(Debug)]
pub enum WdAny {
    Obj(Object),
    Func(Function)
}
pub type Any = Rc<RefCell<WdAny>>;