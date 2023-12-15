use crate::interpreter::*;
use maplit;

pub fn buildin_bool(state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__and__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_and)))
        )),
        "__or__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_or)))
        )),
        "__eq__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_eq)))
        )),
        "__ne__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_ne)))
        )),
        "__not__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_not)))
        )),
        "__init__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(bool_init)))
        )),
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?
    };
    utils::set_attr(
        state.clone(),
        "bool", 
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Not,
                attrs: attrs
            }
    ))))?;
    utils::set_attr(
        state.clone(),
        "true",
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Bool(true),
                attrs: maplit::hashmap! {
                    "__type__".to_string() => utils::get_buildin_var("bool", state.clone())?
                }
            }
    ))))?;
    utils::set_attr(
        state.clone(), 
        "false",
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Bool(false),
                attrs: maplit::hashmap! {
                    "__type__".to_string() => utils::get_buildin_var("bool", state.clone())?
                }
            }
    ))))?;
    Ok(())
}

fn any2bool(x: Any) -> Option<bool> {
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

fn bool_init(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    match args.len() {
        0 => utils::get_buildin_var("false", state),
        1 => {
            let arg = args[0].clone();
            match any2bool(arg.clone()) {
                Some(_) => Ok(arg),
                None => match utils::get_attr(arg.clone(), "__bool__") {
                    Some(tf) => utils::call(tf, args, state),
                    None => bail!("cannot convert {:?} to bool", arg)
                },
            }
        },
        _ => bail!("bool accepts only one argument")
    }
}

fn bool_and(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 && b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute and value of '{:?}' and '{:?}'", _left, _right)
    }
}

fn bool_or(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 || b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compute or value of '{:?}' and '{:?}'", _left, _right)
    }
}

fn bool_eq(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 == b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compare '{:?}' and '{:?}'", _left, _right)
    }
}

fn bool_ne(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    match (any2bool(_left.clone()), any2bool(_right.clone())) {
        (Some(b1), Some(b2)) => match b1 != b2 {
            true => utils::get_buildin_var("true", state.clone()),
            false => utils::get_buildin_var("false", state.clone()),
        },
        _ => bail!("Cannot compare '{:?}' and '{:?}'", _left, _right)
    }
}

fn bool_not(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let _arg = args[0].clone();
    match any2bool(_arg) {
        Some(b) => match b {
            true => utils::get_buildin_var("false", state.clone()),
            false => utils::get_buildin_var("true", state.clone()),
        },
        _ => unreachable!(),
    }
}