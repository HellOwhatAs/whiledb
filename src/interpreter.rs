pub mod runner;
pub mod utils;
pub mod states;
pub mod obj_none;
pub mod obj_type;
pub mod obj_int;
pub mod obj_float;
pub mod obj_bool;
pub mod obj_string;
pub mod obj_list;
use crate::ast::*;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::rc::Rc;
use num::BigInt;
use maplit;
pub use anyhow::Result;
#[cfg(not(debug_assertions))]
use anyhow::bail;
#[cfg(debug_assertions)]
use panic as bail;
pub use runner::{eval, exec};
pub use states::init_state;
#[derive(Clone)]
pub enum BuildIn {
    Bool(bool),
    Int(BigInt),
    Float(f64),
    String(String),
    Tuple(VecDeque<Any>),
    Not
}
#[derive(Clone)]
pub struct Object {
    pub buildin: BuildIn,
    pub attrs: HashMap<String, Any>
}
#[derive(Clone)]
pub struct DefinedFunction {
    pub args: VecDeque<String>,
    pub body: Rc<Cmd>
}
#[derive(Clone)]
pub struct BuildInFunction(pub fn(VecDeque<Any>, Any) -> Result<Any>);
#[derive(Clone)]
pub enum Function {
    BuildInFunction(BuildInFunction),
    DefinedFunction(DefinedFunction)
}

#[derive(Clone)]
pub enum WdAny {
    Obj(Object),
    Func(String, Function)
}
pub type Any = Rc<RefCell<WdAny>>;


#[macro_export]
macro_rules! method {
    (<$attrs:ident> $($func:ident($state: ident, $($args:ident),*) $body:block)*) => {
        {
            $(
                $attrs.insert(stringify!($func).to_string(), Rc::new(RefCell::new(
                    WdAny::Func(stringify!($func).to_string(), Function::BuildInFunction(BuildInFunction({
                        fn the_method_func(args: VecDeque<Any>, state: Any) -> Result<Any> {
                            let mut count: usize = 0;
                            $(
                                let $args = args[count].clone();
                                count += 1;
                            )*
                            drop(args);
                            let _ = count;
                            let $state = state;
                            $body
                        }
                        the_method_func
                    })))
                )));
            )*
        }
    };
    ($($func:ident($state: ident, $($args:ident),*) $body:block)*) => {
        {
            let mut attrs = std::collections::HashMap::new();
            $(
                attrs.insert(stringify!($func).to_string(), Rc::new(RefCell::new(
                    WdAny::Func(stringify!($func).to_string(), Function::BuildInFunction(BuildInFunction({
                        fn the_method_func(args: VecDeque<Any>, state: Any) -> Result<Any> {
                            let mut count: usize = 0;
                            $(
                                let $args = args[count].clone();
                                count += 1;
                            )*
                            drop(args);
                            let _ = count;
                            let $state = state;
                            $body
                        }
                        the_method_func
                    })))
                )));
            )*
            attrs
        }
    };
}