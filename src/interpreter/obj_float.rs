use maplit;
use crate::{interpreter::*, method};

/// add the type `float` to buildin-state and return the state
pub fn buildin_float(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, arg) {
            match any2float(arg.clone()) {
                Some(_) => Ok(arg),
                None => match utils::get_father_attr(arg.clone(), "__float__") {
                    Some(tf) => utils::call(tf, VecDeque::from([arg]), state),
                    None => bail!("cannot convert arg to float")
                }
            }
        }
        __add__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(float2any(i1 + i2, state)),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__radd__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot add left and right"),
                },
                _ => unreachable!(),
            }
        }
        __sub__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(float2any(i1 - i2, state)),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rsub__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot sub left and right"),
                },
                _ => unreachable!(),
            }
        }
        __mul__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(float2any(i1 * i2, state)),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rmul__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot mul left and right"),
                },
                _ => unreachable!(),
            }
        }
        __div__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(float2any(i1 / i2, state)),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rdiv__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot div left and right"),
                },
                _ => unreachable!(),
            }
        }
        __lt__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 < i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rlt__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare lt left and right"),
                },
                _ => unreachable!(),
            }
        }
        __gt__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 > i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rgt__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare gt left and right"),
                },
                _ => unreachable!(),
            }
        }
        __le__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 <= i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rle__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare le left and right"),
                },
                _ => unreachable!(),
            }
        }
        __ge__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 >= i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rge__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare ge left and right"),
                },
                _ => unreachable!(),
            }
        }
        __eq__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 == i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__req__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare eq left and right"),
                },
                _ => unreachable!(),
            }
        }
        __ne__(state, left, right) {
            match (any2float(left.clone()), any2float(right.clone())) {
                (Some(i1), Some(i2)) => Ok(match i1 != i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(right.clone(), "__rne__") {
                    Some(rf) => utils::call(rf, VecDeque::from([right.clone(), left.clone()]), state),
                    None => bail!("Cannot compare ne left and right"),
                },
                _ => unreachable!(),
            }
        }
        __bool__(state, s) {
            match any2float(s.clone()) {
                Some(f) => match f != 0.0 {
                    true => utils::get_buildin_var("true", state.clone()),
                    false => utils::get_buildin_var("false", state.clone()),
                },
                None => unreachable!()
            }
        }
        __float__(_state, s) {
            Ok(s)
        }
        __int__(state, s) {
            match any2float(s.clone()) {
                Some(f) => Ok(obj_int::bigint2intinstance(BigInt::from(f as usize), state)),
                None => unreachable!()
            }
        }
        __string__(state, s) {
            match any2float(s.clone()) {
                Some(f) => Ok(obj_string::build_string(&f.to_string(), state)),
                None => unreachable!()
            }
        }
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "float", 
        res.clone()
    )?;
    Ok(res)
}

pub fn buildin_float_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("float", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}

pub fn build_float(raw: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Float(raw.parse::<f64>().unwrap()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("float", state.clone()).unwrap()
        }
    })))
}

pub fn any2float(x: Any) -> Option<f64> {
    match &*x.borrow() {
        WdAny::Obj(o) => match o.buildin {
            BuildIn::Float(f) => Some(f),
            _ => None
        },
        _ => None
    }
}

pub fn float2any(f: f64, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Float(f),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("float", state.clone()).unwrap()
        }
    })))
}