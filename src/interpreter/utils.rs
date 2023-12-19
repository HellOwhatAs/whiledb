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

pub fn get_father_attr(obj: Any, key: &str) -> Option<Any> {
    match &*obj.borrow() {
        WdAny::Obj(obj) => {
            match obj.attrs.get("__type__") {
                Some(t) => get_attr(t.clone(), key),
                None => None
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
    let root = match get_self_attr(state.clone(), "/") {
        Some(root) => root,
        None => state.clone(),
    };
    match get_self_attr(root, name) {
        Some(v) => Ok(v),
        None => bail!("Undefined buildin-variable `{}`", name),
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
pub fn call(obj: Any, args: VecDeque<Any>, state: Any) -> Result<Any> {
    match &*obj.clone().borrow() {
        WdAny::Obj(_) => {
            match get_self_attr(obj.clone(), "__init__") {
                Some(f) => {
                    let mut args = args;
                    args.push_front(Rc::new(RefCell::new(WdAny::Obj(Object {
                        buildin: BuildIn::Not,
                        attrs: maplit::hashmap! {
                            "__type__".to_string() => obj
                        }
                    }))));
                    call(f, args, state)
                },
                None => match get_self_attr(obj.clone(), "__call__") {
                    Some(f) => call(f, args, state),
                    None => match get_father_attr(obj.clone(), "__call__") {
                        Some(f) => {
                            let mut args = args;
                            args.push_front(obj.clone());
                            call(f, args, state)
                        },
                        None => bail!("cannot call obj")
                    }
                },
            }
        },
        WdAny::Func(_fname, f) => {
            match f {
                Function::BuildInFunction(f) => {
                    (f.0)(args, state)
                },
                Function::DefinedFunction(f) => {
                    let local = local_state(state.clone());
                    for (k, v) in std::iter::zip(f.args.clone(), args) {
                        set_attr(local.clone(), &k, v)?;
                    }
                    let (_, _, ret) = exec(f.body.clone(), local)?;
                    Ok(ret.unwrap_or(get_buildin_var("None", state)?))
                },
            }
        },
    }
}

pub fn local_state(state: Any) -> Any {
    let mut attrs = maplit::hashmap! { "..".to_string() => state.clone() };
    match get_self_attr(state.clone(), "/") {
        Some(root) => attrs.insert("/".to_string(), root),
        None => attrs.insert("/".to_string(), state.clone()),
    };
    let local = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    local
}

pub fn convert2string(arg: Any, state: Any) -> Result<String> {
    match utils::get_father_attr(arg.clone(), "__string__") {
        Some(f) => {
            let s = utils::call(f, VecDeque::from([arg]), state.clone())?;
            match obj_string::any2string(s) {
                Some(s) => Ok(s),
                None => unreachable!(),
            }
        },
        None => bail!("converting an object that cannot convert to string"),
    }
}