use num::{BigInt, Zero};
use maplit;
use crate::{interpreter::*, method};

/// add the type `int` to buildin-state and return the state
pub fn buildin_int(state: Any) -> Result<Any> {
    let attrs = method!{
        __init__(state, _self, __arg) {
            let _arg = __arg.clone();
            let arg = &*_arg.borrow();
            match wdany2bigint(arg) {
                Some(_) => Ok(__arg),
                None => match utils::get_father_attr(__arg.clone(), "__int__") {
                    Some(f) => utils::call(f, VecDeque::from([__arg]), state),
                    None => bail!("cannot convert arg to int"),
                }
            }
        }
        __add__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 + i2, state)),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__radd__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot add left and right"),
                },
                _ => unreachable!(),
            }
        }
        __sub__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 - i2, state)),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rsub__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot sub left and right"),
                },
                _ => unreachable!(),
            }
        }
        __mul__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 * i2, state)),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rmul__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot mul left and right"),
                },
                _ => unreachable!(),
            }
        }
        __div__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(bigint2intinstance(
                    if num::Zero::is_zero(i2) {
                        bail!("Cannot div zero")
                    } else { i1 / i2 }, state)),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rdiv__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot div left and right"),
                },
                _ => unreachable!(),
            }
        }
        __mod__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(bigint2intinstance(
                    if num::Zero::is_zero(i2) {
                        bail!("Cannot mod zero")
                    } else { i1 % i2 }, state)),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rmod__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot mod left and right"),
                },
                _ => unreachable!(),
            }
        }
        __lt__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 < i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rlt__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare lt left and right"),
                },
                _ => unreachable!(),
            }
        }
        __gt__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 > i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rgt__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare gt left and right"),
                },
                _ => unreachable!(),
            }
        }
        __le__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 <= i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rle__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare le left and right"),
                },
                _ => unreachable!(),
            }
        }
        __ge__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 >= i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rge__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare ge left and right"),
                },
                _ => unreachable!(),
            }
        }
        __eq__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 == i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__req__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare eq left and right"),
                },
                _ => unreachable!(),
            }
        }
        __ne__(state, _left, _right) {
            let (left, right) = (&*_left.borrow(), &*_right.borrow());
            match (wdany2bigint(left), wdany2bigint(right)) {
                (Some(i1), Some(i2)) => Ok(match i1 != i2 {
                    true => utils::get_buildin_var("true", state.clone())?,
                    false => utils::get_buildin_var("false", state.clone())?,
                }),
                (Some(_), None) => match utils::get_father_attr(_right.clone(), "__rne__") {
                    Some(rf) => utils::call(rf, VecDeque::from([_right.clone(), _left.clone()]), state),
                    None => bail!("Cannot compare ne left and right"),
                },
                _ => unreachable!(),
            }
        }
        __negate__(state, _arg) {
            let arg = &*_arg.borrow();
            match wdany2bigint(arg) {
                Some(i) => Ok(bigint2intinstance(-i, state.clone())),
                _ => unreachable!(),
            }
        }
        __bool__(state, _arg) {
            let arg = &*_arg.borrow();
            match wdany2bigint(arg) {
                Some(i) => match i.is_zero() {
                    true => utils::get_buildin_var("false", state),
                    false => utils::get_buildin_var("true", state),
                },
                _ => unreachable!(),
            }
        }
        __int__(_state, arg) {
            Ok(arg)
        }
        __string__(state, _arg) {
            let arg = &*_arg.borrow();
            match wdany2bigint(arg) {
                Some(i) => Ok(obj_string::build_string(&i.to_string(), state)),
                _ => unreachable!(),
            }
        }
    };
    let res = Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Not,
        attrs: attrs
    })));
    utils::set_attr(
        state, 
        "int", 
        res.clone()
    )?;
    Ok(res)
}

pub fn buildin_int_post(type_obj: Any, state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?,
        "__name__".to_string() => obj_string::build_string("int", state.clone())
    };
    for (k, v) in attrs.into_iter() {
        utils::set_attr(type_obj.clone(), &k, v)?;
    }
    Ok(())
}

pub fn build_int(raw: &str, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object {
        buildin: BuildIn::Int(raw.parse::<BigInt>().unwrap()),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
        }
    })))
}

fn wdany2bigint(x: &WdAny) -> Option<&BigInt> {
    match x {
        WdAny::Obj(o) => {
            match &o.buildin {
                BuildIn::Int(i) => Some(i),
                _ => None
            }
        },
        _ => None
    }
}

pub fn bigint2intinstance(x: BigInt, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Int(x),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
        }
    })))
}