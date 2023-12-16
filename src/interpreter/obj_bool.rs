use crate::interpreter::*;
use maplit;

pub fn buildin_bool(state: Any) -> Result<Any> {
    let attrs = maplit::hashmap! {
        "__and__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__and__".to_string(), Function::BuildInFunction(BuildInFunction(bool_and)))
        )),
        "__or__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__or__".to_string(), Function::BuildInFunction(BuildInFunction(bool_or)))
        )),
        "__eq__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__eq__".to_string(), Function::BuildInFunction(BuildInFunction(bool_eq)))
        )),
        "__ne__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__ne__".to_string(), Function::BuildInFunction(BuildInFunction(bool_ne)))
        )),
        "__not__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__not__".to_string(), Function::BuildInFunction(BuildInFunction(bool_not)))
        )),
        "__init__".to_string() => Rc::new(RefCell::new(
            WdAny::Func("__init__".to_string(), Function::BuildInFunction(BuildInFunction(bool_init)))
        )),
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state.clone(),
        "bool", 
        res.clone()
    )?;
    utils::set_attr(
        state.clone(),
        "true",
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Bool(true),
                attrs: maplit::hashmap! {
                    "__type__".to_string() => utils::get_buildin_var("bool", state.clone())?
                }
        })))
    )?;
    utils::set_attr(
        state.clone(), 
        "false",
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Bool(false),
                attrs: maplit::hashmap! {
                    "__type__".to_string() => utils::get_buildin_var("bool", state.clone())?
                }
        })))
    )?;
    Ok(res)
}

pub fn any2bool(x: Any) -> Option<bool> {
    match &*x.borrow() {
        WdAny::Obj(o) => {
            match o.buildin {
                BuildIn::Bool(b) => Some(b),
                _ => None
            }
        },
        _ => None,
    }
}

pub fn bool_init(args: VecDeque<Any>, state: Any) -> Result<Any> {
    match args.len() {
        1 => utils::get_buildin_var("false", state),
        2 => {
            let arg = args[1].clone();
            match any2bool(arg.clone()) {
                Some(_) => Ok(arg),
                None => match utils::get_attr(arg.clone(), "__bool__") {
                    Some(tf) => {
                        let mut args = args;
                        args.pop_front();
                        utils::call(tf, args, state)
                    },
                    None => bail!("cannot convert arg to bool")
                },
            }
        },
        _ => bail!("__init__ of bool accepts at most 2 argument")
    }
}

fn bool_and(args: VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 && b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute and value of _left and _right")
    }
}

fn bool_or(args: VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 || b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute or value of _left and _right")
    }
}

fn bool_eq(args: VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 == b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute eq value of _left and _right")
    }
}

fn bool_ne(args: VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 != b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute ne value of _left and _right")
    }
}

fn bool_not(args: VecDeque<Any>, state: Any) -> Result<Any> {
    let _arg = args[0].clone();
    match any2bool(_arg) {
        Some(b) => match b {
            true => utils::get_buildin_var("false", state.clone()),
            false => utils::get_buildin_var("true", state.clone()),
        },
        _ => unreachable!(),
    }
}