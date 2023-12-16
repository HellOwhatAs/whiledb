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

pub enum BuildIn {
    Bool(bool),
    Int(BigInt),
    Float(f64),
    String(String),
    Tuple(VecDeque<Any>),
    Not
}

pub struct Object {
    pub buildin: BuildIn,
    pub attrs: HashMap<String, Any>
}

pub struct DefinedFunction {
    args: Rc<Expr>,
    body: Rc<Cmd>
}

pub struct BuildInFunction(fn(VecDeque<Any>, Any) -> Result<Any>);

pub enum Function {
    BuildInFunction(BuildInFunction),
    DefinedFunction(DefinedFunction)
}

pub enum WdAny {
    Obj(Object),
    Func(String, Function)
}
pub type Any = Rc<RefCell<WdAny>>;