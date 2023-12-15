use crate::interpreter::*;

/// first get attr from a object
/// 
/// if not exist, get attrs from the type object refed in `__type__` attr
pub fn get_attr(obj: Any, key: &str) -> Option<Any> {
    match &*obj.borrow() {
        WdAny::Obj(obj) => {
            match obj.attrs.get(key) {
                Some(v) => Some(v.clone()),
                None => get_attr(obj.attrs.get("__type__")?.clone(), key)
            }
        },
        _ => None,
    }
}

/// just get the attr with no recursive
pub fn get_self_attr(obj: Any, key: &str) -> Option<Any> {
    match &*obj.borrow() {
        WdAny::Obj(obj) => {
            match obj.attrs.get(key) {
                Some(v) => Some(v.clone()),
                None => None
            }
        },
        _ => None,
    }
}

/// get var from local (if not exist nonlocal)
pub fn get_var(name: &str, state: Any) -> Result<Any> {
    match &*state.borrow() {
        WdAny::Obj(obj) => {
            match obj.attrs.get(name) {
                Some(v) => Ok(v.clone()),
                None => match obj.attrs.get("..") {
                    Some(nonlocal) => get_var(name, nonlocal.clone()),
                    None => bail!("Undefined variable `{}`", name)
                }
            }
        },
        _ => unreachable!(),
    }
}

/// get var from the root state
pub fn get_buildin_var(name: &str, state: Any) -> Result<Any> {
    match &*state.borrow() {
        WdAny::Obj(obj) => {
            match obj.attrs.get("..") {
                Some(nonlocal) => get_buildin_var(name, nonlocal.clone()),
                None => match obj.attrs.get(name) {
                    Some(res) => Ok(res.clone()),
                    None => bail!("Undefined buildin-variable `{}`", name)
                }
            }
        },
        _ => unreachable!(),
    }
}

/// set attr of a object
pub fn set_attr(obj: Any,  key: &str, val: Any) -> Result<()> {
    match &mut *obj.borrow_mut() {
        WdAny::Obj(obj) => {
            obj.attrs.insert(key.to_string(), val.clone());
            Ok(())
        },
        _ => bail!("Cannot set attr a function")
    }
}

/// call a `Any` function
/// 
/// either function or a object with `__call__` attr
pub fn call(obj: Any, args: &VecDeque<Any>, state: Any) -> Result<Any> {
    match &*obj.clone().borrow() {
        WdAny::Obj(_) => {
            match get_self_attr(obj.clone(), "__init__") {
                Some(f) => call(f, args, state),
                None => match get_attr(obj.clone(), "__call__") {
                    Some(f) => call(f, args, state),
                    None => bail!("cannot call {:?}", obj)
                },
            }
        },
        WdAny::Func(f) => {
            match f {
                Function::BuildInFunction(f) => {
                    (f.0)(args, state)
                },
                Function::DefinedFunction(f) => todo!(),
            }
        },
    }
}