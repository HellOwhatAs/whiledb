use crate::{interpreter::*, method};
use maplit;

pub fn buildin_bool(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, arg) {
            match any2bool(arg.clone()) {
                Some(_) => Ok(arg),
                None => match utils::get_father_attr(arg.clone(), "__bool__") {
                    Some(tf) => utils::call(tf, VecDeque::from([arg]), state),
                    None => bail!("cannot convert arg to bool")
                },
            }
        }
        __and__(state, left, right) {
            match (any2bool(left.clone()), any2bool(right.clone())) {
                (Some(b1), Some(b2)) => match b1 && b2 {
                    true => utils::get_buildin_var("true", state.clone()),
                    false => utils::get_buildin_var("false", state.clone()),
                },
                _ => bail!("Cannot compute and value of left and right")
            }
        }
        __or__(state, left, right) {
            match (any2bool(left.clone()), any2bool(right.clone())) {
                (Some(b1), Some(b2)) => match b1 || b2 {
                    true => utils::get_buildin_var("true", state.clone()),
                    false => utils::get_buildin_var("false", state.clone()),
                },
                _ => bail!("Cannot compute or value of left and right")
            }
        }
        __eq__(state, left, right) {
            match (any2bool(left.clone()), any2bool(right.clone())) {
                (Some(b1), Some(b2)) => match b1 == b2 {
                    true => utils::get_buildin_var("true", state.clone()),
                    false => utils::get_buildin_var("false", state.clone()),
                },
                _ => bail!("Cannot compute eq value of left and right")
            }
        }
        __ne__(state, left, right) {
            match (any2bool(left.clone()), any2bool(right.clone())) {
                (Some(b1), Some(b2)) => match b1 != b2 {
                    true => utils::get_buildin_var("true", state.clone()),
                    false => utils::get_buildin_var("false", state.clone()),
                },
                _ => bail!("Cannot compute ne value of left and right")
            }
        }
        __not__(state, arg) {
            match any2bool(arg) {
                Some(b) => match b {
                    true => utils::get_buildin_var("false", state.clone()),
                    false => utils::get_buildin_var("true", state.clone()),
                },
                _ => unreachable!(),
            }
        }
        __bool__(_state, s) {
            Ok(s)
        }
        __int__(state, s) {
            match any2bool(s) {
                Some(b) => Ok(obj_int::bigint2intinstance(BigInt::from(b as isize), state)),
                None => unreachable!(),
            }
        }
        __float__(state, s) {
            match any2bool(s) {
                Some(b) => Ok(obj_float::float2any(b as usize as f64, state)),
                None => unreachable!(),
            }
        }
        __string__(state, s) {
            match any2bool(s) {
                Some(b) => Ok(obj_string::build_string(&b.to_string(), state)),
                None => unreachable!(),
            }
        }
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

pub fn buildin_bool_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("bool", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
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