use crate::ast::*;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};
use std::rc::Rc;
use num_bigint::BigInt;
mod obj_int;

#[derive(Debug)]
pub enum BuildIn {
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
pub struct BuildInFunction(fn(VecDeque<Any>, Any) -> (Any, Any));

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

pub fn init_state() -> Any {
    let initial_attrs = HashMap::new();
    Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: initial_attrs
    })))
}

pub fn exec(ast: Rc<Cmd>, state: Any) -> Result<Any, String> {
    match ast.as_ref() {
        Cmd::Asgn(e1, e2) => {
            match e1.as_ref() {
                Expr::Var(s) => {
                    let (v2, state) = eval(e2.clone(), state)?;
                    match &mut *state.clone().borrow_mut() {
                        WdAny::Obj(o) => {
                            o.attrs.insert(s.clone(), v2);
                            Ok(state)
                        },
                        _ => unreachable!(),
                    }
                },
                _ => {
                    let (v1, state) = eval(e1.clone(), state)?;
                    let (v2, state) = eval(e2.clone(), state)?;
                    if Rc::strong_count(&v1) == 1 {
                        return Err(format!("Cannot assign to {:?}", e1));
                    }
                    let _ = std::mem::replace(&mut v1.borrow_mut(), v2.borrow_mut());
                    Ok(state)
                }
            }
        },
        Cmd::Seq(cs) => {
            let mut state = state;
            for c in cs.iter() {
                state = exec(c.clone(), state)?;
            }
            Ok(state)
        },
        Cmd::If(_, _, _) => todo!(),
        Cmd::While(_, _) => todo!(),
        Cmd::Expr(_) => todo!(),
        Cmd::Continue => todo!(),
        Cmd::Break => todo!(),
        Cmd::Func(_, _, _) => todo!(),
        Cmd::Class(_, _) => todo!(),
        Cmd::Return(_) => todo!(),
        Cmd::Nop => Ok(state),
    }
}

pub fn eval(expr: Rc<Expr>, state: Any) -> Result<(Any, Any), String> {
    match expr.as_ref() {
        Expr::ConstInt(s) => Ok(obj_int::build_int(s, state)),
        Expr::ConstFloat(_) => todo!(),
        Expr::ConstString(_) => todo!(),
        Expr::Tuple(_) => todo!(),
        Expr::Var(_) => todo!(),
        Expr::BinOp(_, _, _) => todo!(),
        Expr::UnOp(_, _) => todo!(),
        Expr::Call(_, _) => todo!(),
        Expr::GetItem(_, _) => todo!(),
        Expr::GetAttr(_, _) => todo!(),
    }
}