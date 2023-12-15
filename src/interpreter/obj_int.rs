use num::{BigInt, Zero};
use maplit;
use crate::interpreter::*;

/// add the type `int` to buildin-state and return the state
pub fn buildin_int(state: Any) -> Result<()> {
    let attrs = maplit::hashmap! {
        "__add__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_add)))
        )),
        "__sub__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_sub)))
        )),
        "__mul__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_mul)))
        )),
        "__div__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_div)))
        )),
        "__mod__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_mod)))
        )),
        "__lt__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_lt)))
        )),
        "__gt__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_gt)))
        )),
        "__le__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_le)))
        )),
        "__ge__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_ge)))
        )),
        "__eq__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_eq)))
        )),
        "__ne__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_ne)))
        )),
        "__negate__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_negate)))
        )),
        "__bool__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_2bool)))
        )),
        "__init__".to_string() => Rc::new(RefCell::new(
            WdAny::Func(Function::BuildInFunction(BuildInFunction(int_init)))
        )),
        "__type__".to_string() => utils::get_buildin_var("type", state.clone())?
    };
    utils::set_attr(
        state, 
        "int", 
        Rc::new(RefCell::new(WdAny::Obj(Object{
                buildin: BuildIn::Not,
                attrs: attrs
            }
    ))))
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

fn bigint2intinstance(x: BigInt, state: Any) -> Any {
    Rc::new(RefCell::new(WdAny::Obj(Object{
        buildin: BuildIn::Int(x),
        attrs: maplit::hashmap! {
            "__type__".to_string() => utils::get_buildin_var("int", state.clone()).unwrap()
        }
    })))
}

fn int_add(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 + i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__radd__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot add {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_sub(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 - i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rsub__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot sub {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_mul(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(i1 * i2, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rmul__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot mul {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_div(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(
            if num::Zero::is_zero(i2) {
                bail!("Cannot div zero")
            } else { i1 / i2 }, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rdiv__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot div {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_mod(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(bigint2intinstance(
            if num::Zero::is_zero(i2) {
                bail!("Cannot mod zero")
            } else { i1 % i2 }, state)),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rmod__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot mod {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_lt(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 < i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rlt__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_gt(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 > i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rgt__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_le(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 <= i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rle__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_ge(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 >= i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rge__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_eq(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 == i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__req__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_ne(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let (_left, _right) = (args[0].clone(), args[1].clone());
    let (left, right) = (&*_left.borrow(), &*_right.borrow());
    match (wdany2bigint(left), wdany2bigint(right)) {
        (Some(i1), Some(i2)) => Ok(match i1 != i2 {
            true => utils::get_buildin_var("true", state.clone())?,
            false => utils::get_buildin_var("false", state.clone())?,
        }),
        (Some(_), None) => match utils::get_attr(_right.clone(), "__rne__") {
            Some(rf) => utils::call(rf, &VecDeque::from([_right.clone(), _left.clone()]), state),
            None => bail!("Cannot compare {:?} and {:?}", left, right),
        },
        _ => unreachable!(),
    }
}

fn int_negate(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let _arg = args[0].clone();
    let arg = &*_arg.borrow();
    match wdany2bigint(arg) {
        Some(i) => Ok(bigint2intinstance(-i, state.clone())),
        _ => unreachable!(),
    }
}

fn int_2bool(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    let _arg = args[0].clone();
    let arg = &*_arg.borrow();
    match wdany2bigint(arg) {
        Some(i) => match i.is_zero() {
            true => utils::get_buildin_var("false", state),
            false => utils::get_buildin_var("true", state),
        },
        _ => unreachable!(),
    }
}

fn int_init(args: &VecDeque<Any>, state: Any) -> Result<Any> {
    match args.len() {
        0 => Ok(bigint2intinstance(BigInt::from(0), state)),
        1 => {
            let _arg = args[0].clone();
            let arg = &*_arg.borrow();
            match arg {
                WdAny::Obj(o) => {
                    match o.buildin {
                        BuildIn::Bool(b) => Ok(bigint2intinstance(BigInt::from(b), state)),
                        BuildIn::Int(_) => Ok(args[0].clone()),
                        _ => match utils::get_attr(args[0].clone(), "__int__") {
                            Some(tf) => utils::call(tf, args, state),
                            None => bail!("cannot convert {:?} to bool", arg),
                        }
                    }
                },
                _ => bail!("cannot convert function to int"),
            }
        },
        _ => bail!("int accepts only one argument")
    }
}