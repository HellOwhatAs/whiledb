use crate::interpreter::*;

/// find type from any state
pub fn state2typeobj(name: &str, state: Any) -> Any {
    match &mut *state.clone().borrow_mut() {
        WdAny::Obj(o) => {
            if o.attrs.contains_key("..") {
                state2typeobj(name, state)
            }
            else {
                o.attrs.get(name).unwrap().clone()
            }
        },
        _ => unreachable!()
    }
}

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

pub fn set_attr(obj: Any,  key: &str, val: Any) -> Result<(), String> {
    match &mut *obj.borrow_mut() {
        WdAny::Obj(obj) => {
            obj.attrs.insert(key.to_string(), val.clone());
            Ok(())
        },
        _ => Err("Cannot set attr a function".to_string())
    }
}

pub fn call(obj: Any, args: VecDeque<Any>, state: Any) -> Result<Any, String> {
    match &*obj.clone().borrow() {
        WdAny::Obj(_) => {
            match get_attr(obj.clone(), "__call__") {
                Some(f) => call(f, args, state),
                None => Err(format!("Cannot Call {:?}", obj)),
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